use std::io::Cursor;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use crate::networking::data_types::{BufferWrite, PacketWrite, StreamExt, StreamWrite};
use crate::networking::data_types::var_int::VarInt;
use crate::networking::packets::{PacketRegistry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionPhase {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

pub enum NetMessage {
    SendPacket(Vec<u8>),
    Disconnect,
}

pub struct Connection {
    read_stream: OwnedReadHalf,
    writer_sender: mpsc::Sender<NetMessage>,
    pub phase: ConnectionPhase,
    is_alive: bool,
}

impl Connection {
    pub(crate) fn new(stream: TcpStream, phase: ConnectionPhase) -> Connection {
        let (read_stream, write_stream) = stream.into_split();
        let (tx, rx) = mpsc::channel(128); // 128 packets

        tokio::spawn(handle_writes(write_stream, rx));

        Connection { read_stream, writer_sender: tx, phase, is_alive: true }
    }

    pub async fn close(&mut self) -> anyhow::Result<()> {
        self.is_alive = false;
        let _ = self.writer_sender.send(NetMessage::Disconnect).await;
        Ok(())
    }

    pub(crate) async fn run(&mut self, registry: &Arc<PacketRegistry>) -> anyhow::Result<()> {
        loop {
            if !self.is_alive {
                break;
            }

            let packet_length_res = self.read_stream.read_type::<VarInt>().await;
            let packet_length = match packet_length_res {
                Ok(len) => len,
                Err(_) => {
                    break; // Disconnected
                }
            };

            let mut packet_buffer = vec![0u8; packet_length.0 as usize];
            self.read_stream.read_exact(&mut packet_buffer).await?;
            let mut cursor = Cursor::new(&packet_buffer[..]);

            let packet_id: VarInt = cursor.read_type().await?;

            registry.handle_packet(self.phase, packet_id.0, &mut cursor, self).await?;
        }

        Ok(())
    }

    pub async fn send_packet<T: PacketWrite>(&mut self, id: i32, packet: T) -> anyhow::Result<()> {
        // Create a buffer for the Packet Body (ID + Data)
        let mut body_buffer = Vec::new();
        body_buffer.write_type(VarInt(id));
        body_buffer.write_type(packet);

        // Calculate total length
        let length = VarInt(body_buffer.len() as i32);
        let mut final_buffer = Vec::new();
        final_buffer.write_type(length);
        final_buffer.extend_from_slice(&body_buffer);

        let _ = self.writer_sender.send(NetMessage::SendPacket(final_buffer)).await;
        Ok(())
    }

    pub async fn send_raw_packet(&mut self, packet_id: i32, body: &[u8]) -> anyhow::Result<()> {
        // Packet buffer, ID + Body
        let mut payload_buffer = Vec::with_capacity(5 + body.len());

        // Write Packet ID
        payload_buffer.write_type(VarInt(packet_id));

        // Write Raw Body
        payload_buffer.extend_from_slice(body);

        let length = VarInt(payload_buffer.len() as i32);
        let mut final_buffer = Vec::with_capacity(5 + payload_buffer.len());

        // Write Length Header
        final_buffer.write_type(length);

        // Append the payload
        final_buffer.append(&mut payload_buffer);

        // Send to the Writer Task via the Channel
        self.writer_sender
            .send(NetMessage::SendPacket(final_buffer))
            .await
            .map_err(|_| anyhow::anyhow!("Writer task closed (Connection dropped)"))?;

        Ok(())
    }
}

async fn handle_writes(mut stream: OwnedWriteHalf, mut rx: mpsc::Receiver<NetMessage>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            NetMessage::SendPacket(data) => {
                if stream.write_all(&data).await.is_err() {
                    break; // Client disconnected
                }
            }
            NetMessage::Disconnect => {
                let _ = stream.shutdown().await;
                break;
            }
        }
    }
}

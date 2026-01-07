use std::io::Cursor;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::networking::data_types::{BufferWrite, PacketWrite, StreamExt};
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

pub struct Connection {
    pub(crate) stream: TcpStream,
    pub phase: ConnectionPhase,
    is_alive: bool,
}

impl Connection {
    pub(crate) fn new(stream: TcpStream, phase: ConnectionPhase) -> Connection {
        Connection { stream, phase, is_alive: true }
    }

    pub async fn close(&mut self) -> anyhow::Result<()> {
        self.is_alive = false;
        self.stream.shutdown().await?;
        Ok(())
    }

    pub(crate) async fn run(&mut self, registry: &Arc<PacketRegistry>) -> anyhow::Result<()> {
        loop {
            if !self.is_alive {
                break;
            }

            let packet_length: VarInt = self.stream.read_type().await?;

            let mut packet_buffer = vec![0u8; packet_length.0 as usize];
            self.stream.read_exact(&mut packet_buffer).await?;
            let mut cursor = Cursor::new(&packet_buffer[..]);

            let packet_id: VarInt = cursor.read_type().await?;

            registry.handle_packet(self.phase, packet_id.0, &mut cursor, self).await?;
        }

        Ok(())
    }

    pub async fn send_packet<T: PacketWrite>(&mut self, id: i32, packet: T) -> anyhow::Result<()> {
        // Create a buffer for the Packet Body (ID + Data)
        let mut body_buffer = Vec::new();

        // Write Packet ID
        body_buffer.write_type(VarInt(id));

        // Write Packet Data
        body_buffer.write_type(packet);

        // Calculate total length (Length of Body)
        let length = VarInt(body_buffer.len() as i32);

        // Write Length Header directly to the stream
        let mut length_buffer = Vec::new();
        length_buffer.write_type(length);
        self.stream.write_all(&length_buffer).await?;

        // Write Body
        self.stream.write_all(&body_buffer).await?;

        Ok(())
    }
}


use std::io::Cursor;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::data_types::{BufferWrite, PacketWrite, StreamExt};
use crate::data_types::var_int::VarInt;
use crate::packets::{PacketRegistry};

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

    async fn handle_play(&mut self) -> anyhow::Result<()> {
        // Read "Set Player Position and Rotation - 0x1E"
        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        let x: f64 = self.stream.read_type().await?;
        let feet_y: f64 = self.stream.read_type().await?;
        let z: f64 = self.stream.read_type().await?;
        let yaw: f32 = self.stream.read_type().await?;
        let pitch: f32 = self.stream.read_type().await?;
        let flags: i32 = self.stream.read_type().await?;

        println!("X: {:?}", x);
        println!("Feet Y: {:?}", feet_y);
        println!("Z: {:?}", z);
        println!("Yaw: {:?}", yaw);
        println!("Pitch: {:?}", pitch);
        println!("Flags: {:?}", flags as u32);

        // FROM HERE THE CLIENT STARTS SPAMMING CLIENT TICK PACKETS

        // TODO: Missing another "Player Info Update - 0x44" before, which I need to investigate
        // Send "Player Info Update - 0x44" - Sending that this player joined the server, to tell other online players
        /*let mut buffer = Vec::new();

        buffer.write_type(UnsignedByte(0x01));
        buffer.write_type(UnsignedByte(1));

        send_packet(&mut stream, 0x44, &buffer).await?;*/

        //

        println!("ENTERING PLAY STATE LOOP");
        loop {
            // if the client disconnects, read_type returns an error
            let packet_length: VarInt = match self.stream.read_type().await {
                Ok(v) => v,
                Err(_) => {
                    println!("Client disconnected.");
                    break;
                }
            };

            // packet body
            let mut packet_buffer = vec![0u8; packet_length.0 as usize];
            if self.stream.read_exact(&mut packet_buffer).await.is_err() {
                println!("Failed to read packet body.");
                break;
            }

            let mut cursor = Cursor::new(packet_buffer);

            let packet_id: VarInt = cursor.read_type().await?;

            match packet_id.0 {
                0x01 => {
                    // ?????????????????????????????????
                }

                0x0c => {
                    // Client tick end
                }

                _ => {
                    println!("[Play] Unknown Packet ID: 0x{:02X} (Length: {})", packet_id.0, packet_length.0);
                }
            }
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


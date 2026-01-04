pub(crate) mod handshake;
pub(crate) mod configuration;
pub(crate) mod status;
pub(crate) mod login;
pub(crate) mod play;

use std::collections::HashMap;
use std::io::Cursor;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::data_types::{BufferWrite, StreamWrite};
use crate::data_types::var_int::VarInt;
use anyhow::Result;
use async_trait::async_trait;
use crate::connection::{Connection, ConnectionPhase};

/// # Send packet
/// Generic Packet Sender
/// 1. Writes Packet ID (VarInt) to a buffer
/// 2. Appends Data
/// 3. Calculates total length (ID length + Data length)
/// 4. Prefixes the total length (VarInt)
/// 5. Sends it all
pub async fn send_packet(socket: &mut TcpStream, packet_id: i32, body: &[u8]) -> anyhow::Result<()> {
    let mut packet_buffer = Vec::new();

    // Write Packet ID
    packet_buffer.write_type(VarInt(packet_id));

    // Write Body
    packet_buffer.extend_from_slice(body);

    // Send Total Length + Content
    socket.write_stream_type(VarInt(packet_buffer.len() as i32)).await?;
    socket.write_all(&packet_buffer).await?;

    Ok(())
}

pub trait Packet: Sized {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self>;
}

#[async_trait]
pub trait PacketHandler {
    async fn handle(&self, context: &mut Connection) -> Result<()>;
}

type PacketBuilder = Box<dyn Fn(&mut Cursor<&[u8]>) -> Result<Box<dyn PacketHandler + Send + Sync>> + Send + Sync>;

pub struct PacketRegistry {
    handlers: HashMap<(ConnectionPhase, i32), PacketBuilder>,
}
impl PacketRegistry {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }

    pub fn register<P>(&mut self, phase: ConnectionPhase, id: i32)
    where
        P: Packet + PacketHandler + Send + Sync + 'static,
    {
        // The closure only needs to know how to decode P
        let builder = Box::new(|cursor: &mut Cursor<&[u8]>| {
            // 1. Decode specific packet (Sync)
            let packet = P::decode(cursor)?;

            // 2. Return it as a Trait Object
            // We cast "PluginMessagePacket" into "Box<dyn PacketHandler>"
            Ok(Box::new(packet) as Box<dyn PacketHandler + Send + Sync>)
        });

        self.handlers.insert((phase, id), builder);
    }

    pub async fn handle_packet(
        &self,
        phase: ConnectionPhase,
        id: i32,
        cursor: &mut Cursor<&[u8]>,
        ctx: &mut Connection
    ) -> Result<()> {

        if let Some(builder) = self.handlers.get(&(phase, id)) {
            // 1. Use the builder to decode the packet from bytes
            // Returns Box<dyn PacketHandler>
            let packet_handler = builder(cursor)?;

            // 2. Now run the handler logic
            // Since we are awaiting immediately, the borrow checker knows 'ctx' is safe
            packet_handler.handle(ctx).await

        } else {
            // Handle unknown packet
            println!("Unknown packet: {:?}", id);
            Ok(())
        }
    }
}
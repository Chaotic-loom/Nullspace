use std::io::Cursor;
use async_trait::async_trait;
use crate::connection::{Connection};
use crate::data_types::BufferReadExt;
use crate::packets::{Packet, PacketHandler};
use crate::packets::status::pong_response::PongResponsePacket;

pub struct PingRequestPacket {
    pub timestamp: i64,
}

impl Packet for PingRequestPacket {
    fn decode(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let timestamp: i64 = cursor.read_field()?;

        Ok(PingRequestPacket { timestamp })
    }
}

#[async_trait]
impl PacketHandler for PingRequestPacket {
    async fn handle(&self, ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling ping request...");

        // The client waits for the same timestamp he passed, to verify the connection
        let response = PongResponsePacket {
            timestamp: self.timestamp
        };

        // Send Packet 0x01 (Response)
        ctx.send_packet(0x01, response).await?;

        // Close the connection
        ctx.close().await?;

        Ok(())
    }
}
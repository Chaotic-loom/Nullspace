use std::io::Cursor;
use async_trait::async_trait;
use crate::networking::connection::{Connection, ConnectionPhase};
use crate::networking::packets::{Packet, PacketHandler};

pub struct LoginAcknowledgedRequestPacket {}

impl Packet for LoginAcknowledgedRequestPacket {
    fn decode(_cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        Ok(LoginAcknowledgedRequestPacket { })
    }
}

#[async_trait]
impl PacketHandler for LoginAcknowledgedRequestPacket {
    async fn handle(&self, ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling login acknowledged request...");

        println!("Switching to CONFIGURATION phase");
        ctx.phase = ConnectionPhase::Configuration;

        Ok(())
    }
}
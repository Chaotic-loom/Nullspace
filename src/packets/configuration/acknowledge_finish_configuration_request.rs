use std::io::Cursor;
use async_trait::async_trait;
use crate::connection::{Connection, ConnectionPhase};
use crate::packets::{Packet, PacketHandler};
use crate::packets::play::login_response::LoginResponsePacket;
use crate::packets::play::synchronize_player_position_response::SynchronizePlayerPositionResponsePacket;

pub struct AcknowledgeFinishConfigurationRequestPacket {}

impl Packet for AcknowledgeFinishConfigurationRequestPacket {
    fn decode(_cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        Ok(AcknowledgeFinishConfigurationRequestPacket { })
    }
}

#[async_trait]
impl PacketHandler for AcknowledgeFinishConfigurationRequestPacket {
    async fn handle(&self, ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling acknowledge finished request...");

        println!("Switching to PLAY phase");
        ctx.phase = ConnectionPhase::Play;

        // Send Packets (Responses)
        ctx.send_packet(0x30, LoginResponsePacket::nullspace()).await?;
        ctx.send_packet(0x46, SynchronizePlayerPositionResponsePacket::nullspace()).await?;

        Ok(())
    }
}
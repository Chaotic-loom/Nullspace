use std::io::Cursor;
use async_trait::async_trait;
use uuid::Uuid;
use crate::connection::Connection;
use crate::data_types::BufferReadExt;
use crate::packets::{Packet, PacketHandler};
use crate::packets::login::login_success_response::LoginSuccessResponsePacket;

pub struct LoginStartRequestPacket {
    pub name: String,
    pub player_uuid: Uuid,
}

impl Packet for LoginStartRequestPacket {
    fn decode(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let name: String = cursor.read_field()?;
        let player_uuid: Uuid = cursor.read_field()?;

        Ok(LoginStartRequestPacket { name, player_uuid })
    }
}

#[async_trait]
impl PacketHandler for LoginStartRequestPacket {
    async fn handle(&self, ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling login start request...");

        // Send Packet 0x02 (Response)
        ctx.send_packet(0x02, LoginSuccessResponsePacket::new(
            self.player_uuid,
            self.name.clone(),
            Vec::new()
        )).await?;

        Ok(())
    }
}
use std::io::Cursor;
use async_trait::async_trait;
use crate::connection::Connection;
use crate::data_types::BufferReadExt;
use crate::data_types::var_int::VarInt;
use crate::packets::{Packet, PacketHandler};

pub struct TeleportConfirmationRequestPacket {
    pub teleport_id: VarInt,
}

impl Packet for TeleportConfirmationRequestPacket {
    fn decode(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let teleport_id: VarInt = cursor.read_field()?;

        Ok(TeleportConfirmationRequestPacket { teleport_id })
    }
}

#[async_trait]
impl PacketHandler for TeleportConfirmationRequestPacket {
    async fn handle(&self, _ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling teleport confirmation request...");

        println!("Teleport ID: {:?}", self.teleport_id);

        Ok(())
    }
}
use std::io::Cursor;
use async_trait::async_trait;
use crate::networking::connection::Connection;
use crate::networking::packets::{Packet, PacketHandler};

pub struct ClientTickEndRequestPacket {}

impl Packet for ClientTickEndRequestPacket {
    fn decode(_cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        Ok(ClientTickEndRequestPacket { })
    }
}

#[async_trait]
impl PacketHandler for ClientTickEndRequestPacket {
    async fn handle(&self, _ctx: &mut Connection) -> anyhow::Result<()> {
        //println!("Handling client tick end request...");

        Ok(())
    }
}
use std::io::Cursor;
use async_trait::async_trait;
use crate::networking::connection::Connection;
use crate::networking::data_types::BufferReadExt;
use crate::networking::data_types::known_pack::KnownPack;
use crate::networking::packets::{Packet, PacketHandler};
use crate::networking::packets::configuration::finish_configuration_response::FinishConfigurationResponsePacket;
use crate::networking::packets::configuration::registry_response::send_all_registries;

pub struct KnownPacksRequestPacket {
    pub known_packs: Vec<KnownPack>,
}

impl Packet for KnownPacksRequestPacket {
    fn decode(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let known_packs: Vec<KnownPack> = cursor.read_field()?;

        Ok(KnownPacksRequestPacket { known_packs })
    }
}

#[async_trait]
impl PacketHandler for KnownPacksRequestPacket {
    async fn handle(&self, ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling known packs request...");

        println!("Known packs: {:?}", self.known_packs);

        // Send "Registry data - 0x07" and "Update tags - 0x0D"
        send_all_registries(&mut ctx.stream).await?;

        // Send "Finish configuration - 0x03"
        ctx.send_packet(0x03, FinishConfigurationResponsePacket { }).await?;

        Ok(())
    }
}
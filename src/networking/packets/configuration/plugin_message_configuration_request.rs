use std::io::Cursor;
use anyhow::{Result};
use async_trait::async_trait;
use crate::networking::connection::Connection;
use crate::networking::data_types::identifier::Identifier;
use crate::networking::data_types::raw_bytes::RawBytes;
use crate::networking::data_types::{BufferReadExt};
use crate::networking::packets::{Packet, PacketHandler};
use crate::networking::packets::configuration::feature_flags_response::FeatureFlagsResponsePacket;
use crate::networking::packets::configuration::known_packs_response::KnownPacksResponsePacket;
use crate::networking::packets::configuration::plugin_message_configuration_response::PluginMessageConfigurationResponsePacket;

pub struct PluginMessageConfigurationRequestPacket {
    pub channel: Identifier,
    pub data: RawBytes,
}

impl Packet for PluginMessageConfigurationRequestPacket {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let channel: Identifier = cursor.read_field()?;
        let data: RawBytes = cursor.read_field()?;

        Ok(PluginMessageConfigurationRequestPacket { channel, data })
    }
}

#[async_trait]
impl PacketHandler for PluginMessageConfigurationRequestPacket {
    async fn handle(&self, ctx: &mut Connection) -> Result<()> {
        println!("Handling Plugin Message on channel: {}", self.channel);

        if self.channel == Identifier::new("minecraft", "brand") {
            let mut brand_cursor = Cursor::new(&self.data.0);
            let brand_name: String = brand_cursor.read_field()?;
            println!("Client Brand: {}", brand_name);

            if brand_name != "vanilla" {
                println!("Modded client detected!");
            }
        } else {
            println!("Unknown plugin configuration channel: {}", self.channel);
        }
        
        // Send Packets (Responses)
        ctx.send_packet(0x01, PluginMessageConfigurationResponsePacket::nullspace()).await?;
        ctx.send_packet(0x0C, FeatureFlagsResponsePacket::nullspace()).await?;
        ctx.send_packet(0x0E, KnownPacksResponsePacket::nullspace()).await?;
        
        Ok(())
    }
}
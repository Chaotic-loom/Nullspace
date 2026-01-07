use std::io::Cursor;
use async_trait::async_trait;
use serde_json::json;
use crate::networking::connection::{Connection};
use crate::networking::data_types::BufferWrite;
use crate::networking::packets::{send_packet, Packet, PacketHandler};
use crate::networking::packets::status::status_response::StatusResponsePacket;

pub struct StatusRequestPacket {}

impl Packet for StatusRequestPacket {
    fn decode(_cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        Ok(StatusRequestPacket { })
    }
}

#[async_trait]
impl PacketHandler for StatusRequestPacket {
    async fn handle(&self, ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling status request...");

        let status_response = json!({
            "version": {
                "name": "1.21.11",
                "protocol": 774
            },
            "players": {
                "max": 1000,
                "online": 0,
                "sample": [
                    {
                        "name": "UnknownData",
                        "id": "04984259-3bf4-4551-9a54-7489e53d8be4"
                    }
                ]
            },
            "description": {
                "text": "§b§lNULLSPACE | Rust server\n§r§7Built for 1000+ players."
            }
        });

        let json_str = serde_json::to_string(&status_response)?;

        let response = StatusResponsePacket {
            json_response: json_str
        };

        // Send Packet 0x00 (Response)
        ctx.send_packet(0x00, response).await?;

        Ok(())
    }
}
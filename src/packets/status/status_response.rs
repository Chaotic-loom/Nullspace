use serde::Serialize;
use crate::data_types::{BufferWrite, PacketWrite};
use crate::data_types::var_int::VarInt;

#[derive(Serialize)]
struct StatusResponsePayload {
    version: VersionInfo,
    players: PlayerInfo,
    description: DescriptionInfo,
    //favicon: Option<String>, TODO
    //enforcesSecureChat: Option<bool>, TODO
}

#[derive(Serialize)]
struct VersionInfo {
    name: String,
    protocol: VarInt,
}

#[derive(Serialize)]
struct PlayerInfo {
    max: VarInt,
    online: VarInt,
    sample: Option<Vec<PlayerSample>>,
}

#[derive(Serialize)]
struct PlayerSample {
    name: String,
    id: String,
}

#[derive(Serialize)]
struct DescriptionInfo {
    text: String,
}

pub struct StatusResponsePacket {
    pub json_response: String,
}

impl PacketWrite for StatusResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.json_response.clone());
    }
}
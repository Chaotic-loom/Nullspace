use uuid::Uuid;
use crate::networking::data_types::{BufferWrite, PacketWrite};
use crate::networking::data_types::game_profile::{GameProfile, GameProfileProperty};

pub struct LoginSuccessResponsePacket {
    pub profile: GameProfile,
}

impl PacketWrite for LoginSuccessResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.profile.clone());
    }
}

impl LoginSuccessResponsePacket {
    pub fn new(uuid: Uuid, username: String, properties: Vec<GameProfileProperty>) -> Self {
        Self { profile: GameProfile { uuid, username, properties } }
    }
}
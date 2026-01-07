use crate::networking::data_types::{BufferWrite, PacketWrite};
use crate::networking::data_types::known_pack::KnownPack;

pub struct KnownPacksResponsePacket {
    pub packs: Vec<KnownPack>,
}

impl PacketWrite for KnownPacksResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.packs.clone());
    }
}

impl KnownPacksResponsePacket {
    pub fn nullspace() -> KnownPacksResponsePacket {
        KnownPacksResponsePacket {
            packs: vec![
                KnownPack {
                    namespace: "minecraft".to_string(),
                    id: "core".to_string(),
                    version: "1.21.11".to_string(),
                }
            ]
        }
    }
}
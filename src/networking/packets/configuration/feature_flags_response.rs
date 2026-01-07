use crate::networking::data_types::identifier::Identifier;
use crate::networking::data_types::{BufferWrite, PacketWrite};

pub struct FeatureFlagsResponsePacket {
    pub flags: Vec<Identifier>,
}

impl PacketWrite for FeatureFlagsResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.flags.clone());
    }
}

impl FeatureFlagsResponsePacket {
    pub fn nullspace() -> FeatureFlagsResponsePacket {
        let mut payload_buffer = Vec::new();
        "Nullspace".to_string().write_to(&mut payload_buffer);

        FeatureFlagsResponsePacket {
            flags: vec![
                Identifier::new("minecraft", "vanilla")
            ]
        }
    }
}
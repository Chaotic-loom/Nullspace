use crate::networking::data_types::{PacketWrite};

pub struct FinishConfigurationResponsePacket {}

impl PacketWrite for FinishConfigurationResponsePacket {
    fn write_to(&self, _buf: &mut Vec<u8>) {}
}
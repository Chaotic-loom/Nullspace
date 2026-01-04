use crate::data_types::{BufferWrite, PacketWrite};
use crate::data_types::identifier::Identifier;
use crate::data_types::raw_bytes::RawBytes;

pub struct PluginMessageConfigurationResponsePacket {
    pub channel: Identifier,
    pub data: RawBytes,
}

impl PacketWrite for PluginMessageConfigurationResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.channel.clone());
        buf.write_type(self.data.clone());
    }
}

impl PluginMessageConfigurationResponsePacket {
    pub fn nullspace() -> PluginMessageConfigurationResponsePacket {
        let mut payload_buffer = Vec::new();
        "Nullspace".to_string().write_to(&mut payload_buffer);

        PluginMessageConfigurationResponsePacket {
            channel: Identifier::new("minecraft", "brand"),
            data: RawBytes(payload_buffer),
        }
    }
}
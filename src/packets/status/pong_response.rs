use crate::data_types::{BufferWrite, PacketWrite};

pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl PacketWrite for PongResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.timestamp);
    }
}
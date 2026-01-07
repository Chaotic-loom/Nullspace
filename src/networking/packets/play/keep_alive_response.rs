use std::time::{SystemTime, UNIX_EPOCH};
use crate::networking::data_types::{BufferWrite, PacketWrite};

pub struct KeepAliveResponsePacket {
    pub keep_alive_id: i64,
}

impl PacketWrite for KeepAliveResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.keep_alive_id);
    }
}

impl KeepAliveResponsePacket {
    pub fn new() -> KeepAliveResponsePacket {
        KeepAliveResponsePacket {
            keep_alive_id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as i64,
        }
    }
}
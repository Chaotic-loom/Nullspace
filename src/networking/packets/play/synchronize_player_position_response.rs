use crate::networking::data_types::{BufferWrite, PacketWrite};
use crate::networking::data_types::var_int::VarInt;

pub struct SynchronizePlayerPositionResponsePacket {
    pub teleport_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i32
}

impl PacketWrite for SynchronizePlayerPositionResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.teleport_id);
        buf.write_type(self.x);
        buf.write_type(self.y);
        buf.write_type(self.z);
        buf.write_type(self.velocity_x);
        buf.write_type(self.velocity_y);
        buf.write_type(self.velocity_z);
        buf.write_type(self.yaw);
        buf.write_type(self.pitch);
        buf.write_type(self.flags);
    }
}

impl SynchronizePlayerPositionResponsePacket {
    pub fn nullspace() -> SynchronizePlayerPositionResponsePacket {
        SynchronizePlayerPositionResponsePacket {
            teleport_id: VarInt(0),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        }
    }
}
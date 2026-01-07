use std::io::Cursor;
use async_trait::async_trait;
use crate::networking::connection::Connection;
use crate::networking::data_types::BufferReadExt;
use crate::networking::data_types::i_byte::Byte;
use crate::networking::packets::{Packet, PacketHandler};

pub struct SetPlayerPositionAndRotationRequestPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: Byte,
}

impl Packet for SetPlayerPositionAndRotationRequestPacket {
    fn decode(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let x: f64 = cursor.read_field()?;
        let feet_y: f64 = cursor.read_field()?;
        let z: f64 = cursor.read_field()?;
        let yaw: f32 = cursor.read_field()?;
        let pitch: f32 = cursor.read_field()?;
        let flags: Byte = cursor.read_field()?;

        Ok(SetPlayerPositionAndRotationRequestPacket { x, feet_y, z, yaw, pitch, flags })
    }
}

#[async_trait]
impl PacketHandler for SetPlayerPositionAndRotationRequestPacket {
    async fn handle(&self, _ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling set player position and rotation request...");

        println!("X: {:?}", self.x);
        println!("Feet Y: {:?}", self.feet_y);
        println!("Z: {:?}", self.z);
        println!("Yaw: {:?}", self.yaw);
        println!("Pitch: {:?}", self.pitch);
        println!("Flags: {:?}", self.flags);

        Ok(())
    }
}
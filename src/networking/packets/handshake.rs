use std::io::Cursor;
use async_trait::async_trait;
use crate::networking::connection::{Connection, ConnectionPhase};
use crate::networking::data_types::BufferReadExt;
use crate::networking::data_types::var_int::VarInt;
use crate::networking::packets::{Packet, PacketHandler};

pub struct HandshakePacket {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub port: u16,
    pub next_state: VarInt
}

impl Packet for HandshakePacket {
    fn decode(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let protocol_version: VarInt = cursor.read_field()?;
        let server_address: String = cursor.read_field()?;
        let port: u16 = cursor.read_field()?;
        let next_state: VarInt = cursor.read_field()?;

        Ok(HandshakePacket { protocol_version, server_address, port, next_state })
    }
}

#[async_trait]
impl PacketHandler for HandshakePacket {
    async fn handle(&self, ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling handshake, protocol {:?} and {:?} as intent...", self.protocol_version, self.next_state);

        if self.protocol_version != VarInt::from(774) {
            Err(anyhow::anyhow!("Protocol version mismatch"))?
        }

        match self.next_state.0 {
            1 => {
                println!("Switching to STATUS phase");
                ctx.phase = ConnectionPhase::Status;
            },
            2 => {
                println!("Switching to LOGIN phase");
                ctx.phase = ConnectionPhase::Login;
            },
            _ => return Err(anyhow::anyhow!("Invalid next state intent: {}", self.next_state.0)),
        }

        Ok(())
    }
}
use std::io::Read;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::networking::data_types::{FieldRead, PacketRead, PacketWrite};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VarInt(pub i32);

impl VarInt {
    const MAX_SIZE: usize = 5;
}

impl From<i32> for VarInt {
    fn from(v: i32) -> Self { VarInt(v) }
}
impl From<VarInt> for i32 {
    fn from(v: VarInt) -> Self { v.0 }
}

#[async_trait]
impl PacketRead for VarInt {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut num = 0;
        let mut received = 0;
        loop {
            let mut buf = [0; 1];
            stream.read_exact(&mut buf).await?;
            let byte = buf[0];
            let value = (byte & 0x7F) as i32;
            num |= value << (7 * received);

            received += 1;
            if received > Self::MAX_SIZE {
                return Err(anyhow::anyhow!("VarInt too big"));
            }

            if (byte & 0x80) == 0 {
                return Ok(VarInt(num));
            }
        }
    }
}

impl FieldRead for VarInt {
    fn read_from<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        let mut num = 0;
        let mut received = 0;
        loop {
            let mut buf = [0; 1];
            reader.read_exact(&mut buf)?;
            let byte = buf[0];
            let value = (byte & 0x7F) as i32;
            num |= value << (7 * received);

            received += 1;
            if received > Self::MAX_SIZE {
                return Err(anyhow::anyhow!("VarInt too big"));
            }

            if (byte & 0x80) == 0 {
                return Ok(VarInt(num));
            }
        }
    }
}


impl PacketWrite for VarInt {
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut temp = self.0 as u32;
        loop {
            if (temp & !0x7F) == 0 {
                buf.push(temp as u8);
                return;
            }
            buf.push((temp & 0x7F) as u8 | 0x80);
            temp >>= 7;
        }
    }
}
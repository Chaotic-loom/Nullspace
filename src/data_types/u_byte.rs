use std::io::Read;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::data_types::{FieldRead, PacketRead, PacketWrite};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnsignedByte(pub u8);

#[async_trait]
impl PacketRead for UnsignedByte {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf).await?;
        Ok(UnsignedByte(buf[0]))
    }
}

impl FieldRead for UnsignedByte {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf)?;
        Ok(UnsignedByte(buf[0]))
    }
}

impl PacketWrite for UnsignedByte {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.push(self.0);
    }
}
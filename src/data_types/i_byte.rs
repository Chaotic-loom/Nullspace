use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::data_types::{PacketRead, PacketWrite};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Byte(pub i8);

#[async_trait]
impl PacketRead for Byte {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf).await?;
        Ok(Byte(buf[0] as i8))
    }
}

impl PacketWrite for Byte {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.push(self.0 as u8);
    }
}
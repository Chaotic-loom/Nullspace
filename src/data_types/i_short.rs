use std::io::Read;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::data_types::{FieldRead, PacketRead, PacketWrite};

#[async_trait]
impl PacketRead for i16 {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf).await?;
        Ok(i16::from_be_bytes(buf))
    }
}

impl FieldRead for i16 {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }
}

impl PacketWrite for i16 {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}
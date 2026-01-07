use std::io::Read;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::networking::data_types::{FieldRead, PacketRead, PacketWrite};

#[async_trait]
impl PacketRead for f32 {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).await?;
        Ok(f32::from_be_bytes(buf))
    }
}

impl FieldRead for f32 {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }
}

impl PacketWrite for f32 {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use uuid::Uuid;
use crate::data_types::{PacketRead, PacketWrite};

#[async_trait]
impl PacketRead for Uuid {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 16];
        stream.read_exact(&mut buf).await?;
        Ok(Uuid::from_bytes(buf))
    }
}

impl PacketWrite for Uuid {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }
}
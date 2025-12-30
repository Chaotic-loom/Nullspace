use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::data_types::{PacketRead, PacketWrite};

#[async_trait]
impl PacketRead for i64 {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 8];
        stream.read_exact(&mut buf).await?;
        Ok(i64::from_be_bytes(buf))
    }
}

impl PacketWrite for i64 {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}
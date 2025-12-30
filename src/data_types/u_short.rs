use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::data_types::{PacketRead, PacketWrite};

#[async_trait]
impl PacketRead for u16 {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf).await?;
        Ok(u16::from_be_bytes(buf))
    }
}
impl PacketWrite for u16 {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}
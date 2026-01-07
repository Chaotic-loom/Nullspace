use std::io::Read;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::networking::data_types::{FieldRead, PacketRead, PacketWrite};

#[async_trait]
impl PacketRead for bool {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf).await?;
        Ok(buf[0] != 0)
    }
}

impl FieldRead for bool {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }
}

impl PacketWrite for bool {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.push(if *self { 1 } else { 0 });
    }
}
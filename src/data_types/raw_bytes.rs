use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::data_types::{PacketRead, PacketWrite};

/// Represents raw binary data that consumes the stream until EOF.
/// WARNING: Only use this on a bounded stream (like a Cursor over a fixed buffer).
/// If we use this on a TcpStream, it will hang forever waiting for the connection to close.
#[derive(Debug, Clone)]
pub struct RawBytes(pub Vec<u8>);

#[async_trait]
impl PacketRead for RawBytes {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await?;
        Ok(RawBytes(buf))
    }
}

impl PacketWrite for RawBytes {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.0);
    }
}
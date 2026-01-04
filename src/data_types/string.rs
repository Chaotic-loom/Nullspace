use std::io::Read;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::data_types::{FieldRead, PacketRead, PacketWrite};
use crate::data_types::var_int::VarInt;

#[async_trait]
impl PacketRead for String {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        // We reuse the VarInt logic to read the length!
        let len = <VarInt as PacketRead>::read_from(stream).await?.0;

        if len > 32767 || len < 0 {
            return Err(anyhow::anyhow!("String length invalid"));
        }

        let mut buf = vec![0u8; len as usize];
        stream.read_exact(&mut buf).await?;

        String::from_utf8(buf).map_err(|_| anyhow::anyhow!("Invalid UTF-8"))
    }
}

impl FieldRead for String {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        // We reuse the VarInt logic to read the length!
        let len = <VarInt as FieldRead>::read_from(stream)?.0;

        if len > 32767 || len < 0 {
            return Err(anyhow::anyhow!("String length invalid"));
        }

        let mut buf = vec![0u8; len as usize];
        stream.read_exact(&mut buf)?;

        String::from_utf8(buf).map_err(|_| anyhow::anyhow!("Invalid UTF-8"))
    }
}

impl PacketWrite for String {
    fn write_to(&self, buf: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        // Reuse VarInt write logic for length
        VarInt(bytes.len() as i32).write_to(buf);
        buf.extend_from_slice(bytes);
    }
}
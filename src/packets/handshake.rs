/*use async_trait::async_trait;
use tokio::io::AsyncRead;
use crate::data_types::StreamExt;
use crate::data_types::var_int::VarInt;
use crate::packets::ReadPacket;

pub struct HandshakePacket {
    protocol_version: VarInt,
    server_address: String,
    port: u16,
    next_state: VarInt,
}

#[async_trait]
impl ReadPacket for HandshakePacket {
    async fn decode<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        Ok(HandshakePacket {
            protocol_version: stream.read_type().await?,
            server_address: stream.read_type().await?,
            port: stream.read_type().await?,
            next_state: stream.read_type().await?,
        })
    }
}*/
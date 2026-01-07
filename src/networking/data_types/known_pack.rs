use std::io::Read;
use crate::networking::data_types::{FieldRead, PacketRead, PacketWrite};

#[derive(Debug, Clone, PartialEq)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl PacketWrite for KnownPack {
    fn write_to(&self, buf: &mut Vec<u8>) {
        self.namespace.write_to(buf);
        self.id.write_to(buf);
        self.version.write_to(buf);
    }
}

#[async_trait::async_trait]
impl PacketRead for KnownPack {
    async fn read_from<R: tokio::io::AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        Ok(KnownPack {
            namespace: <String as PacketRead>::read_from(stream).await?,
            id: <String as PacketRead>::read_from(stream).await?,
            version: <String as PacketRead>::read_from(stream).await?,
        })
    }
}

impl FieldRead for KnownPack {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        Ok(KnownPack {
            namespace: <String as FieldRead>::read_from(stream)?,
            id: <String as FieldRead>::read_from(stream)?,
            version: <String as FieldRead>::read_from(stream)?,
        })
    }
}
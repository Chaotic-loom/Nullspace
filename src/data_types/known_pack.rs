use crate::data_types::{PacketRead, PacketWrite};

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
            namespace: String::read_from(stream).await?,
            id: String::read_from(stream).await?,
            version: String::read_from(stream).await?,
        })
    }
}
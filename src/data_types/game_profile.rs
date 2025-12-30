use uuid::Uuid;
use crate::data_types::{PacketRead, PacketWrite};

#[derive(Debug, Clone)]
pub struct GameProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl PacketWrite for GameProfileProperty {
    fn write_to(&self, buf: &mut Vec<u8>) {
        self.name.write_to(buf);
        self.value.write_to(buf);
        self.signature.write_to(buf);
    }
}

#[async_trait::async_trait]
impl PacketRead for GameProfileProperty {
    async fn read_from<R: tokio::io::AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        Ok(GameProfileProperty {
            name: String::read_from(stream).await?,
            value: String::read_from(stream).await?,
            signature: Option::<String>::read_from(stream).await?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<GameProfileProperty>,
}

impl PacketWrite for GameProfile {
    fn write_to(&self, buf: &mut Vec<u8>) {
        self.uuid.write_to(buf);
        self.username.write_to(buf);
        self.properties.write_to(buf);
    }
}

#[async_trait::async_trait]
impl PacketRead for GameProfile {
    async fn read_from<R: tokio::io::AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        Ok(GameProfile {
            uuid: Uuid::read_from(stream).await?,
            username: String::read_from(stream).await?,
            properties: Vec::<GameProfileProperty>::read_from(stream).await?,
        })
    }
}
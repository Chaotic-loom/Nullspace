pub(crate) mod var_int;
mod boolean;
mod u_short;
mod i_integer;
mod string;
pub(crate) mod u_byte;
mod i_short;
mod i_long;
mod float;
mod double;
mod uuid;
mod prefixed_array;
pub(crate) mod game_profile;
pub(crate) mod known_pack;
pub(crate) mod identifier;
pub(crate) mod raw_bytes;
pub(crate) mod i_byte;
pub(crate) mod registries;
pub(crate) mod position;

use std::io::Read;
use tokio::io::{AsyncRead};
use async_trait::async_trait;
use anyhow::Result;
use tokio::io::AsyncWriteExt;

// Encoding & Decoding

#[async_trait]
pub trait PacketWrite {
    fn write_to(&self, buf: &mut Vec<u8>);
}

#[async_trait]
pub trait PacketRead: Sized {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> Result<Self>;
}

// Extending reading
pub trait StreamExt {
    async fn read_type<T: PacketRead>(&mut self) -> anyhow::Result<T>;
}

impl<R: AsyncRead + Unpin + Send> StreamExt for R {
    async fn read_type<T: PacketRead>(&mut self) -> anyhow::Result<T> {
        T::read_from(self).await
    }
}

// Extending writing

pub trait BufferWrite {
    fn write_type<T: PacketWrite>(&mut self, value: T);
}

// 2. Implement it for Vec<u8>
impl BufferWrite for Vec<u8> {
    fn write_type<T: PacketWrite>(&mut self, value: T) {
        value.write_to(self);
    }
}

#[async_trait::async_trait]
pub trait StreamWrite {
    async fn write_stream_type<T: PacketWrite + Send + Sync>(&mut self, value: T) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<W: tokio::io::AsyncWrite + Unpin + Send> StreamWrite for W {
    async fn write_stream_type<T: PacketWrite + Send + Sync>(&mut self, value: T) -> anyhow::Result<()> {
        let mut buf = Vec::new();
        value.write_to(&mut buf);

        self.write_all(&buf).await?;
        Ok(())
    }
}

// Optional signatures

impl<T: PacketWrite> PacketWrite for Option<T> {
    fn write_to(&self, buf: &mut Vec<u8>) {
        match self {
            Some(val) => {
                true.write_to(buf);
                val.write_to(buf);
            }
            None => {
                false.write_to(buf);
            }
        }
    }
}

#[async_trait]
impl<T: PacketRead + Send> PacketRead for Option<T> {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> Result<Self> {
        let has_value = <bool as PacketRead>::read_from(stream).await?;
        if has_value {
            let val = T::read_from(stream).await?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }
}

impl<T: FieldRead> FieldRead for Option<T> {
    fn read_from<R: Read>(stream: &mut R) -> Result<Self> {
        let has_value = <bool as FieldRead>::read_from(stream)?;
        if has_value {
            let val = T::read_from(stream)?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }
}

// 1. Define a Sync version of PacketRead
pub trait FieldRead: Sized {
    fn read_from<R: Read>(reader: &mut R) -> anyhow::Result<Self>;
}

// 2. Define a Sync Extension (like your StreamExt)
pub trait BufferReadExt {
    fn read_field<T: FieldRead>(&mut self) -> anyhow::Result<T>;
}

// 3. Implement the extension for any Sync Reader (like Cursor)
impl<R: Read> BufferReadExt for R {
    fn read_field<T: FieldRead>(&mut self) -> anyhow::Result<T> {
        T::read_from(self)
    }
}
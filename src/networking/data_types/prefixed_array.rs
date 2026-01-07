use std::io::Read;
use crate::networking::data_types::{FieldRead, PacketRead, PacketWrite};
use async_trait::async_trait;
use tokio::io::AsyncRead;
use crate::networking::data_types::var_int::VarInt;

// WRITING: Length (VarInt) + Items
impl<T: PacketWrite> PacketWrite for Vec<T> {
    fn write_to(&self, buf: &mut Vec<u8>) {
        // Write the length as VarInt
        let len = VarInt(self.len() as i32);
        len.write_to(buf);

        // Write every item
        for item in self {
            item.write_to(buf);
        }
    }
}

// READING: Length (VarInt) + Items
#[async_trait]
impl<T: PacketRead + Send> PacketRead for Vec<T> {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        // Read the length
        let len = <VarInt as PacketRead>::read_from(stream).await?;
        let count = i32::from(len);

        // Read items
        let mut items = Vec::with_capacity(count as usize);
        for _ in 0..count {
            items.push(T::read_from(stream).await?);
        }

        Ok(items)
    }
}

impl<T: FieldRead> FieldRead for Vec<T> {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        // Read the length
        let len = <VarInt as FieldRead>::read_from(stream)?;
        let count = i32::from(len);

        // Read items
        let mut items = Vec::with_capacity(count as usize);
        for _ in 0..count {
            items.push(T::read_from(stream)?);
        }

        Ok(items)
    }
}
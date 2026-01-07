use std::io::Read;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::networking::data_types::{FieldRead, PacketRead, PacketWrite};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32, // 26 bits
    pub z: i32, // 26 bits
    pub y: i16, // 12 bits
}

impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Encodes the Position into the Minecraft 1.14+ i64 format.
    /// Format: ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF)
    fn to_u64(&self) -> u64 {
        let x = self.x as i64;
        let z = self.z as i64;
        let y = self.y as i64;

        // Masking is crucial here to handle negative numbers correctly when ORing bits
        (
            ((x & 0x3FFFFFF) << 38) |
                ((z & 0x3FFFFFF) << 12) |
                (y & 0xFFF)
        ) as u64
    }

    /// Decodes the Position from a Minecraft 1.14+ i64.
    /// Relies on arithmetic shifts (>>) for sign extension.
    fn from_u64(val: u64) -> Self {
        // We cast to i64 to use arithmetic shifts (preserves sign)
        let val = val as i64;

        // x: Top 26 bits.
        // Right shift 38 moves them to the bottom.
        let x = (val >> 38) as i32;

        // y: Bottom 12 bits.
        // Shift left 52 (clears top 52 bits), then right 52 (restores position + sign extends).
        let y = (val << 52 >> 52) as i16;

        // z: Middle 26 bits.
        // Shift left 26 (clears top X bits), then right 38 (moves Z to bottom + sign extends).
        let z = (val << 26 >> 38) as i32;

        Position { x, y, z }
    }
}

#[async_trait]
impl PacketRead for Position {
    async fn read_from<R: AsyncRead + Unpin + Send>(stream: &mut R) -> anyhow::Result<Self> {
        // Position is sent as a Long (8 bytes)
        let mut buf = [0u8; 8];
        stream.read_exact(&mut buf).await?;
        let val = u64::from_be_bytes(buf);
        Ok(Position::from_u64(val))
    }
}

impl FieldRead for Position {
    fn read_from<R: Read>(stream: &mut R) -> anyhow::Result<Self> {
        let mut buf = [0u8; 8];
        stream.read_exact(&mut buf)?;
        let val = u64::from_be_bytes(buf);
        Ok(Position::from_u64(val))
    }
}

impl PacketWrite for Position {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_u64().to_be_bytes());
    }
}
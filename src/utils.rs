use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// # VarInt
/// Variable length integer (1-5 bytes)

const MAX_VARINT_SIZE: usize = 5;

pub async fn read_varint<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<i32> {
    let mut num = 0;
    let mut received = 0;
    loop {
        let mut buf = [0; 1];
        stream.read_exact(&mut buf).await?;
        let byte = buf[0];
        let value = (byte & 0x7F) as i32;
        num |= value << (7 * received);

        received += 1;
        if received > MAX_VARINT_SIZE {
            return Err(anyhow::anyhow!("VarInt too big"));
        }

        if (byte & 0x80) == 0 {
            return Ok(num);
        }
    }
}

pub fn write_varint(val: i32, buf: &mut Vec<u8>) {
    let mut temp = val as u32;
    loop {
        if (temp & !0x7F) == 0 {
            buf.push(temp as u8);
            return;
        }
        buf.push((temp & 0x7F) as u8 | 0x80);
        temp >>= 7;
    }
}

/// # Unsigned Short
/// 2 bytes, Big Endian

pub async fn read_u16<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<u16> {
    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf).await?;
    // from_be_bytes = Big Endian
    Ok(u16::from_be_bytes(buf))
}

pub fn write_u16(val: u16, buf: &mut Vec<u8>) {
    // to_be_bytes = Big Endian
    buf.extend_from_slice(&val.to_be_bytes());
}

/// # Signed Short
/// 2 bytes, Big Endian

pub async fn read_i16<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<i16> {
    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf).await?;
    Ok(i16::from_be_bytes(buf))
}

pub fn write_i16(val: i16, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&val.to_be_bytes());
}

/// # Boolean
/// 1 byte: 0x00 = false, 0x01 = true

pub async fn read_bool<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<bool> {
    let mut buf = [0u8; 1];
    stream.read_exact(&mut buf).await?;
    Ok(buf[0] != 0)
}

pub fn write_bool(val: bool, buf: &mut Vec<u8>) {
    buf.push(if val { 1 } else { 0 });
}

/// # Bytes

pub async fn read_u8<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<u8> {
    let mut buf = [0u8; 1];
    stream.read_exact(&mut buf).await?;
    Ok(buf[0])
}

pub fn write_u8(val: u8, buf: &mut Vec<u8>) {
    buf.push(val);
}

/// # Signed Int
/// 4 bytes, Big Endian.

pub async fn read_i32<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<i32> {
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).await?;
    Ok(i32::from_be_bytes(buf))
}

pub fn write_i32(val: i32, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&val.to_be_bytes());
}

/// # Signed Long
/// 8 bytes, Big Endian

pub async fn read_i64<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<i64> {
    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf).await?;
    Ok(i64::from_be_bytes(buf))
}

pub fn write_i64(val: i64, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&val.to_be_bytes());
}

/// # Float

pub async fn read_f32<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<f32> {
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).await?;
    Ok(f32::from_be_bytes(buf))
}

pub fn write_f32(val: f32, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&val.to_be_bytes());
}

/// # Double

pub async fn read_f64<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<f64> {
    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf).await?;
    Ok(f64::from_be_bytes(buf))
}

pub fn write_f64(val: f64, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&val.to_be_bytes());
}

/// # String
/// Prefixed by a VarInt length, then UTF-8 bytes.

pub async fn read_string<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<String> {
    let len = read_varint(stream).await?;

    // Safety limit (32767 is what I found, TODO: research properly)
    if len > 32767 || len < 0 {
        return Err(anyhow::anyhow!("String length invalid"));
    }

    let mut buf = vec![0u8; len as usize];
    stream.read_exact(&mut buf).await?;

    let s = String::from_utf8(buf).map_err(|_| anyhow::anyhow!("Invalid UTF-8"))?;
    Ok(s)
}

pub fn write_string(val: &str, buf: &mut Vec<u8>) {
    let bytes = val.as_bytes();
    write_varint(bytes.len() as i32, buf);
    buf.extend_from_slice(bytes);
}

/// # UUID
/// 128-bit integer (16 bytes)..

pub async fn read_uuid<R: AsyncReadExt + Unpin>(stream: &mut R) -> anyhow::Result<u128> {
    let mut buf = [0u8; 16];
    stream.read_exact(&mut buf).await?;
    Ok(u128::from_be_bytes(buf))
}

pub fn write_uuid(val: u128, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&val.to_be_bytes());
}

/// # Send packet
/// Generic Packet Sender
/// 1. Writes Packet ID (VarInt) to a buffer
/// 2. Appends Data
/// 3. Calculates total length (ID length + Data length)
/// 4. Prefixes the total length (VarInt)
/// 5. Sends it all
pub async fn send_packet(socket: &mut TcpStream, packet_id: i32, body: &[u8]) -> anyhow::Result<()> {
    let mut packet_content = Vec::new();

    // Write Packet ID
    write_varint(packet_id, &mut packet_content);

    // Write Body
    packet_content.extend_from_slice(body);

    // Send Total Length + Content
    let mut final_buffer = Vec::new();
    write_varint(packet_content.len() as i32, &mut final_buffer);
    final_buffer.extend_from_slice(&packet_content);

    socket.write_all(&final_buffer).await?;
    Ok(())
}
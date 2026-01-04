pub(crate) mod handshake;

use tokio::io::{AsyncRead, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::data_types::{BufferWrite, StreamWrite};
use crate::data_types::var_int::VarInt;

/// # Send packet
/// Generic Packet Sender
/// 1. Writes Packet ID (VarInt) to a buffer
/// 2. Appends Data
/// 3. Calculates total length (ID length + Data length)
/// 4. Prefixes the total length (VarInt)
/// 5. Sends it all
pub async fn send_packet(socket: &mut TcpStream, packet_id: i32, body: &[u8]) -> anyhow::Result<()> {
    let mut packet_buffer = Vec::new();

    // Write Packet ID
    packet_buffer.write_type(VarInt(packet_id));

    // Write Body
    packet_buffer.extend_from_slice(body);

    // Send Total Length + Content
    socket.write_stream_type(VarInt(packet_buffer.len() as i32)).await?;
    socket.write_all(&packet_buffer).await?;

    Ok(())
}
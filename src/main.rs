use anyhow::anyhow;
use tokio::net::{TcpListener, TcpStream};
use serde_json::json;
use tokio::io::{AsyncWriteExt};
use crate::data_types::{BufferWrite, StreamExt, StreamWrite};

mod data_types;

use crate::data_types::var_int::VarInt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:25565").await?;
    println!("Server running on 0.0.0.0:25565 (Target: 1.20.1 / Proto: 763)");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                println!("Connection error: {:?}", e);
            }
        });
    }
}

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

// Packet length
// Packet ID: 0x00
// Protocol Version: VarInt
// Server Address: String(255)
// Server Port: Unsigned Short
// Intent: VarInt Enum (1 = Status, 2 = Login, 3 = Transfer)
async fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    println!("Connection established");

    let packet_length: VarInt = stream.read_type().await?;
    let packet_id: VarInt = stream.read_type().await?;

    println!("Packet length: {:?}", packet_length);
    println!("Packet ID: {:?}", packet_id);

    // HANDSHAKE PACKET
    if packet_id == VarInt::from(0x00) {
        let protocol_version: VarInt = stream.read_type().await?;
        let server_address: String = stream.read_type().await?;
        let port: u16 = stream.read_type().await?;
        let next_state: VarInt = stream.read_type().await?;

        println!("Protocol version: {:?}", protocol_version);
        println!("Server address: {}", server_address);
        println!("Port: {}", port);
        println!("Next state: {:?}", next_state);

        if next_state == VarInt::from(1) {
            return handle_status(stream).await;
        } else if next_state == VarInt::from(2) {
            return handle_login(stream).await;
        }
    }

    Ok(())
}

async fn handle_login(mut stream: TcpStream) -> anyhow::Result<()> {
    let packet_length: VarInt = stream.read_type().await?;
    let packet_id: VarInt = stream.read_type().await?;

    Ok(())
}

async fn handle_status(mut stream: TcpStream) -> anyhow::Result<()> {
    let packet_length: VarInt = stream.read_type().await?;
    let packet_id: VarInt = stream.read_type().await?;

    println!("Packet length: {:?}", packet_length);
    println!("Packet ID: {:?}", packet_id);

    // If not handshake, end connection
    if packet_id != VarInt::from(0x00) {
        return Err(anyhow!("Unexpected packet ID during Status phase: Got {:?}. Expected 0x00.", packet_id));
    }

    // Prepare JSON Response
    let status_response = json!({
        "version": {
            "name": "1.21.11",
            "protocol": 774
        },
        "players": {
            "max": 1000,
            "online": 0,
            "sample": []
        },
        "description": {
            "text": "§b§lNULLSPACE | Rust server\n§r§7Built for 1000+ players."
        }
    });

    let json_str = serde_json::to_string(&status_response)?;

    // Create the body
    let mut response_body_buffer = Vec::new();
    response_body_buffer.write_type(json_str);

    println!("Response body: {:?}", response_body_buffer);

    // Send Packet 0x00 (Response)
    send_packet(&mut stream, 0x00, &response_body_buffer).await?;

    // HANDLE PING/PONG
    // Client sends Packet 0x01 (Ping) with a Long payload
    let _packet_length: VarInt = stream.read_type().await?;
    let packet_id: VarInt = stream.read_type().await?;

    if packet_id == VarInt::from(0x01) {
        // Read the Long (payload)
        let payload: i64 = stream.read_type().await?;

        // Write the Long back (Echo)
        let mut pong_body_buffer = Vec::new();
        pong_body_buffer.write_type(payload);

        // Send Packet 0x01 (Pong)
        send_packet(&mut stream, 0x01, &pong_body_buffer).await?;
    }

    Ok(())
}
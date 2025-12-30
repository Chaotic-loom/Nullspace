use std::io::Cursor;
use anyhow::anyhow;
use tokio::net::{TcpListener, TcpStream};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use crate::data_types::{BufferWrite, PacketWrite, StreamExt, StreamWrite};
use crate::data_types::game_profile::GameProfile;
use crate::data_types::i_byte::Byte;
use crate::data_types::identifier::Identifier;
use crate::data_types::known_pack::KnownPack;
use crate::data_types::raw_bytes::RawBytes;
use crate::data_types::u_byte::UnsignedByte;

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
    // 2
    let packet_length: VarInt = stream.read_type().await?;
    let packet_id: VarInt = stream.read_type().await?;

    println!("Packet length: {:?}", packet_length);
    println!("Packet ID: {:?}", packet_id);

    let username: String = stream.read_type().await?;
    let uuid: Uuid = stream.read_type().await?;

    println!("Username: {:?}", username);
    println!("UUID: {:?}", uuid);

    // 3
    let mut buffer = Vec::new();

    buffer.write_type(GameProfile {
        uuid,
        username,
        properties: Vec::new(),
    });

    send_packet(&mut stream, 0x02, &buffer).await?;

    // 4
    let packet_length: VarInt = stream.read_type().await?;
    let packet_id: VarInt = stream.read_type().await?;

    println!("Packet length: {:?}", packet_length);
    println!("Packet ID: {:?}", packet_id);

    // 5
    let packet_length: VarInt = stream.read_type().await?;

    let mut packet_buffer = vec![0u8; packet_length.0 as usize];
    stream.read_exact(&mut packet_buffer).await?;
    let mut cursor = Cursor::new(packet_buffer);

    let packet_id: VarInt = cursor.read_type().await?;

    println!("Packet length: {:?}", packet_length);
    println!("Packet ID: {:?}", packet_id);

    let channel_identifier: Identifier = cursor.read_type().await?;
    let data: RawBytes = cursor.read_type().await?;

    println!("Channel identifier: {:?}", channel_identifier);
    println!("Data: {:?}", data);

    if channel_identifier == "minecraft:brand".parse()? {
        let mut brand_cursor = Cursor::new(data.0);
        let brand_name: String = brand_cursor.read_type().await?;
        println!("Client Brand: {}", brand_name);
    }

    // 6
    let packet_length: VarInt = stream.read_type().await?;
    let packet_id: VarInt = stream.read_type().await?;

    println!("Packet length: {:?}", packet_length);
    println!("Packet ID: {:?}", packet_id);

    let locale: String = stream.read_type().await?;
    let view_distance: Byte = stream.read_type().await?;
    let chat_mode: VarInt = stream.read_type().await?;
    let chat_colors: bool = stream.read_type().await?;
    let displayed_skin_parts: UnsignedByte = stream.read_type().await?;
    let main_hand: VarInt = stream.read_type().await?;
    let enable_text_filtering: bool = stream.read_type().await?;
    let allow_server_listings: bool = stream.read_type().await?;
    let particle_status: VarInt = stream.read_type().await?;

    println!("Locale: {:?}", locale);
    println!("View distance: {:?}", view_distance);
    println!("Chat mode: {:?}", chat_mode);
    println!("Chat colors: {:?}", chat_colors);
    println!("Displayed skin parts: {:?}", displayed_skin_parts);
    println!("Main hand: {:?}", main_hand);
    println!("Enable text filtering: {:?}", enable_text_filtering);
    println!("Allow server listings: {:?}", allow_server_listings);
    println!("Particle status: {:?}", particle_status);

    // 7
    let mut payload_buffer = Vec::new();
    "Nullspace".to_string().write_to(&mut payload_buffer);

    let mut packet_body = Vec::new();

    let identifier = Identifier::new("minecraft", "brand");
    identifier.write_to(&mut packet_body);

    RawBytes(payload_buffer).write_to(&mut packet_body);

    send_packet(&mut stream, 0x01, &packet_body).await?;

    // 8
    let mut buffer = Vec::new();

    buffer.write_type(vec![
        Identifier::new("minecraft", "vanilla")
    ]);

    send_packet(&mut stream, 0x0C, &buffer).await?;

    //9
    let mut buffer = Vec::new();

    buffer.write_type(vec![
        KnownPack {
            namespace: "minecraft".to_string(),
            id: "core".to_string(),
            version: "1.21.11".to_string(),
        }
    ]);

    send_packet(&mut stream, 0x0E, &buffer).await?;

    // 10
    let packet_length: VarInt = stream.read_type().await?;
    let packet_id: VarInt = stream.read_type().await?;

    println!("Packet length: {:?}", packet_length);
    println!("Packet ID: {:?}", packet_id);

    let known_packs: Vec<KnownPack> = stream.read_type().await?;

    println!("Known packs: {:?}", known_packs);

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
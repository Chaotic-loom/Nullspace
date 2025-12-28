use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;

mod utils;
use utils::{read_varint, write_varint};
use crate::utils::{read_i64, read_string, read_u16, send_packet, write_i64, write_string};

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

// Packet length
// Packet ID: 0x00
// Protocol Version: VarInt
// Server Address: String(255)
// Server Port: Unsigned Short
// Intent: VarInt Enum (1 = Status, 2 = Login, 3 = Transfer)
async fn handle_connection(mut socket: TcpStream) -> anyhow::Result<()> {
    println!("Connection established");

    let packet_length = read_varint(&mut socket).await?;
    let packet_id = read_varint(&mut socket).await?;

    println!("Packet length: {}", packet_length);
    println!("Packet ID: {}", packet_id);

    // HANDSHAKE PACKET
    if packet_id == 0x00 {
        let protocol_version = read_varint(&mut socket).await?;

        // Server Address (String)
        let server_address = read_string(&mut socket).await?;

        // Server Port (Unsigned Short)
        let port = read_u16(&mut socket).await?;

        // Intent (1 = Status, 2 = Login, 3 = Transfer)
        let next_state = read_varint(&mut socket).await?;

        println!("Protocol version: {}", protocol_version);
        println!("Server address: {}", server_address);
        println!("Port: {}", port);
        println!("Next state: {}", next_state);

        if next_state == 1 {
            return handle_status(socket).await;
        } else if next_state == 2 {
            // TODO: Login implementation
            return Ok(());
        }
    }

    Ok(())
}

async fn handle_status(mut socket: TcpStream) -> anyhow::Result<()> {
    let packet_length = read_varint(&mut socket).await?;
    let packet_id = read_varint(&mut socket).await?;

    println!("Packet length: {}", packet_length);
    println!("Packet ID: {}", packet_id);

    // Prepare JSON Response
    let status_response = json!({
        "version": {
            "name": "1.20.1",
            "protocol": 763
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
    let mut response_body = Vec::new();
    write_string(&json_str, &mut response_body);

    println!("Response body: {:?}", response_body);

    // Send Packet 0x00 (Response)
    send_packet(&mut socket, 0x00, &response_body).await?;

    // HANDLE PING/PONG
    // Client sends Packet 0x01 (Ping) with a Long payload
    let _len = read_varint(&mut socket).await?;
    let packet_id = read_varint(&mut socket).await?;

    if packet_id == 0x01 {
        // Read the Long (payload)
        let payload = read_i64(&mut socket).await?;

        // Write the Long back (Echo)
        let mut pong_body = Vec::new();
        write_i64(payload, &mut pong_body);

        // Send Packet 0x01 (Pong)
        send_packet(&mut socket, 0x01, &pong_body).await?;
    }

    Ok(())
}
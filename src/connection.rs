use std::collections::HashSet;
use std::io::Cursor;
use std::time::Duration;
use anyhow::anyhow;
use serde_json::json;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::time::sleep;
use uuid::Uuid;
use crate::data_types::{BufferWrite, PacketWrite, StreamExt};
use crate::data_types::game_profile::GameProfile;
use crate::data_types::i_byte::Byte;
use crate::data_types::identifier::Identifier;
use crate::data_types::known_pack::KnownPack;
use crate::data_types::raw_bytes::RawBytes;
use crate::data_types::registries::RegistryData;
use crate::data_types::u_byte::UnsignedByte;
use crate::data_types::var_int::VarInt;
use crate::packets::send_packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionPhase {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

pub struct Connection {
    stream: TcpStream,
    phase: ConnectionPhase,
}

impl Connection {
    pub(crate) fn new(stream: TcpStream, phase: ConnectionPhase) -> Connection {
        Connection { stream, phase }
    }

    pub(crate) fn get_phase(&self) -> ConnectionPhase {
        self.phase
    }

    pub(crate) async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            match self.phase {
                ConnectionPhase::Handshaking => self.handle_handshake().await?,
                ConnectionPhase::Status => {
                    self.handle_status().await?;
                    return Ok(());
                }
                ConnectionPhase::Login => self.handle_login().await?,
                ConnectionPhase::Configuration => self.handle_configuration().await?,
                ConnectionPhase::Play => {
                    self.handle_play().await?;
                    return Ok(());
                }
            }
        }
    }

    async fn handle_handshake(&mut self) -> anyhow::Result<()> {
        println!("Handling handshake");

        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        // HANDSHAKE PACKET
        if packet_id == VarInt::from(0x00) {
            let protocol_version: VarInt = self.stream.read_type().await?;
            let server_address: String = self.stream.read_type().await?;
            let port: u16 = self.stream.read_type().await?;
            let next_state: VarInt = self.stream.read_type().await?;

            println!("Protocol version: {:?}", protocol_version);
            println!("Server address: {}", server_address);
            println!("Port: {}", port);
            println!("Next state: {:?}", next_state);

            // 1 = Status/MOTD
            // 2 = Join/Play
            // 3 = Transfer
            if next_state == VarInt::from(1) {
                self.phase = ConnectionPhase::Status;
            } else if next_state == VarInt::from(2) {
                self.phase = ConnectionPhase::Login;
            }
        }

        Ok(())
    }

    async fn handle_status(&mut self) -> anyhow::Result<()> {
        println!("Handling status");

        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        // If not handshake, end connection
        if packet_id != VarInt::from(0x00) {
            return Err(anyhow::anyhow!("Unexpected packet ID during Status phase: Got {:?}. Expected 0x00.", packet_id));
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
        send_packet(&mut self.stream, 0x00, &response_body_buffer).await?;

        // HANDLE PING/PONG
        // Client sends Packet 0x01 (Ping) with a Long payload
        let _packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        if packet_id == VarInt::from(0x01) {
            // Read the Long (payload)
            let payload: i64 = self.stream.read_type().await?;

            // Write the Long back (Echo)
            let mut pong_body_buffer = Vec::new();
            pong_body_buffer.write_type(payload);

            // Send Packet 0x01 (Pong)
            send_packet(&mut self.stream, 0x01, &pong_body_buffer).await?;
        }

        Ok(())
    }

    async fn handle_login(&mut self) -> anyhow::Result<()> {
        println!("Handling login");

        // Read "Login start - 0x00"
        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        let username: String = self.stream.read_type().await?;
        let uuid: Uuid = self.stream.read_type().await?;

        println!("Username: {:?}", username);
        println!("UUID: {:?}", uuid);

        // Send "Login Success - 0x02"
        let mut buffer = Vec::new();

        buffer.write_type(GameProfile {
            uuid,
            username,
            properties: Vec::new(),
        });

        send_packet(&mut self.stream, 0x02, &buffer).await?;

        // Read "Login acknowledged - 0x03"
        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        println!("Login Acknowledged. Switching to Configuration.");
        self.phase = ConnectionPhase::Configuration;

        Ok(())
    }

    async fn handle_configuration(&mut self) -> anyhow::Result<()> {
        println!("Handling configuration");

        // Read "Plugin message configuration - 0x02"
        let packet_length: VarInt = self.stream.read_type().await?;

        let mut packet_buffer = vec![0u8; packet_length.0 as usize];
        self.stream.read_exact(&mut packet_buffer).await?;
        let mut cursor = Cursor::new(packet_buffer);

        let packet_id: VarInt = cursor.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        let channel_identifier: Identifier = cursor.read_type().await?;
        let data: RawBytes = cursor.read_type().await?;

        println!("Channel identifier: {:?}", channel_identifier);
        println!("Data: {:?}", data);

        let mut is_modded = false;

        if channel_identifier == "minecraft:brand".parse()? {
            let mut brand_cursor = Cursor::new(data.0);
            let brand_name: String = brand_cursor.read_type().await?;
            println!("Client Brand: {}", brand_name);

            if brand_name != "vanilla" {
                is_modded = true;
            }
        }

        if is_modded {
            return Err(anyhow!("Modded packets are not supported yet!"));
        }

        // Read "Client information - 0x00"
        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        let locale: String = self.stream.read_type().await?;
        let view_distance: Byte = self.stream.read_type().await?;
        let chat_mode: VarInt = self.stream.read_type().await?;
        let chat_colors: bool = self.stream.read_type().await?;
        let displayed_skin_parts: UnsignedByte = self.stream.read_type().await?;
        let main_hand: VarInt = self.stream.read_type().await?;
        let enable_text_filtering: bool = self.stream.read_type().await?;
        let allow_server_listings: bool = self.stream.read_type().await?;
        let particle_status: VarInt = self.stream.read_type().await?;

        println!("Locale: {:?}", locale);
        println!("View distance: {:?}", view_distance);
        println!("Chat mode: {:?}", chat_mode);
        println!("Chat colors: {:?}", chat_colors);
        println!("Displayed skin parts: {:?}", displayed_skin_parts);
        println!("Main hand: {:?}", main_hand);
        println!("Enable text filtering: {:?}", enable_text_filtering);
        println!("Allow server listings: {:?}", allow_server_listings);
        println!("Particle status: {:?}", particle_status);

        // Send "Plugin message configuration - 0x01"
        let mut payload_buffer = Vec::new();
        "Nullspace".to_string().write_to(&mut payload_buffer);

        let mut packet_body = Vec::new();

        let identifier = Identifier::new("minecraft", "brand");
        identifier.write_to(&mut packet_body);

        RawBytes(payload_buffer).write_to(&mut packet_body);

        send_packet(&mut self.stream, 0x01, &packet_body).await?;

        // Send "Feature flags - 0x0C"
        let mut buffer = Vec::new();

        buffer.write_type(vec![
            Identifier::new("minecraft", "vanilla")
        ]);

        send_packet(&mut self.stream, 0x0C, &buffer).await?;

        // Send "Known packs - 0x0E"
        let mut buffer = Vec::new();

        buffer.write_type(vec![
            KnownPack {
                namespace: "minecraft".to_string(),
                id: "core".to_string(),
                version: "1.21.11".to_string(),
            }
        ]);

        send_packet(&mut self.stream, 0x0E, &buffer).await?;

        // Read "Known packs - 0x07"
        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        let known_packs: Vec<KnownPack> = self.stream.read_type().await?;

        println!("Known packs: {:?}", known_packs);

        // Send "Registry data - 0x07" and "Update tags - 0x0D"
        println!("Sending registries!");

        send_all_registries(&mut self.stream).await?;

        // Send "Finish configuration - 0x03"
        println!("Sending 'Finish configuration'!");

        let buf = Vec::new();
        send_packet(&mut self.stream, 0x03, &buf).await?;

        // Read "Acknowledge finish configuration - 0x03"
        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        println!("Configuration Acknowledged. Switching to Play.");
        self.phase = ConnectionPhase::Play;

        Ok(())
    }

    async fn handle_play(&mut self) -> anyhow::Result<()> {
        println!("Handling play loop");

        // Send "Login (play) - 0x30"
        let mut buffer = Vec::new();

        buffer.write_type(2_i32);
        buffer.write_type(false);
        buffer.write_type(vec![
            Identifier::new("minecraft", "overworld")
        ]);
        buffer.write_type(VarInt::from(1000));
        buffer.write_type(VarInt::from(10));
        buffer.write_type(VarInt::from(10));
        buffer.write_type(false);
        buffer.write_type(true);
        buffer.write_type(false);
        buffer.write_type(VarInt::from(0));
        buffer.write_type(Identifier::new("minecraft", "overworld"));
        buffer.write_type(0_i64);
        buffer.write_type(UnsignedByte(1));
        buffer.write_type(Byte(0));
        buffer.write_type(false); // debug world, true deactives the ability to modify the world, this could be usefull ofr testing i gues
        buffer.write_type(false);
        buffer.write_type(false);
        buffer.write_type(VarInt::from(0));
        buffer.write_type(VarInt::from(64));
        buffer.write_type(true);

        send_packet(&mut self.stream, 0x30, &buffer).await?;

        // Send "Synchronize Player Position - 0x46"
        let mut buffer = Vec::new();

        buffer.write_type(VarInt::from(0));
        buffer.write_type(0_f64);
        buffer.write_type(0_f64);
        buffer.write_type(0_f64);
        buffer.write_type(0_f64);
        buffer.write_type(0_f64);
        buffer.write_type(0_f64);
        buffer.write_type(0_f32);
        buffer.write_type(0_f32);
        buffer.write_type(0x0001_i32);

        send_packet(&mut self.stream, 0x46, &buffer).await?;

        // Read if modded client
        /*if is_modded {
            // plugin, minecraft:register
            sleep(Duration::from_secs(5)).await;
            return Err(anyhow!("Modded packets are not supported yet!"));
        }*/

        // Read "Teleport confirmation - 0x00"
        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        let teleport_id: VarInt = self.stream.read_type().await?;
        println!("Teleport ID: {:?}", teleport_id);

        // Read "Set Player Position and Rotation - 0x1E"
        let packet_length: VarInt = self.stream.read_type().await?;
        let packet_id: VarInt = self.stream.read_type().await?;

        println!("Packet length: {:?}", packet_length);
        println!("Packet ID: {:?}", packet_id);

        let x: f64 = self.stream.read_type().await?;
        let feet_y: f64 = self.stream.read_type().await?;
        let z: f64 = self.stream.read_type().await?;
        let yaw: f32 = self.stream.read_type().await?;
        let pitch: f32 = self.stream.read_type().await?;
        let flags: i32 = self.stream.read_type().await?;

        println!("X: {:?}", x);
        println!("Feet Y: {:?}", feet_y);
        println!("Z: {:?}", z);
        println!("Yaw: {:?}", yaw);
        println!("Pitch: {:?}", pitch);
        println!("Flags: {:?}", flags as u32);

        // FROM HERE THE CLIENT STARTS SPAMMING CLIENT TICK PACKETS

        // TODO: Missing another "Player Info Update - 0x44" before, which I need to investigate
        // Send "Player Info Update - 0x44" - Sending that this player joined the server, to tell other online players
        /*let mut buffer = Vec::new();

        buffer.write_type(UnsignedByte(0x01));
        buffer.write_type(UnsignedByte(1));

        send_packet(&mut stream, 0x44, &buffer).await?;*/

        //

        println!("ENTERING PLAY STATE LOOP");
        loop {
            // if the client disconnects, read_type returns an error
            let packet_length: VarInt = match self.stream.read_type().await {
                Ok(v) => v,
                Err(_) => {
                    println!("Client disconnected.");
                    break;
                }
            };

            // packet body
            let mut packet_buffer = vec![0u8; packet_length.0 as usize];
            if self.stream.read_exact(&mut packet_buffer).await.is_err() {
                println!("Failed to read packet body.");
                break;
            }

            let mut cursor = Cursor::new(packet_buffer);

            let packet_id: VarInt = cursor.read_type().await?;

            match packet_id.0 {
                0x01 => {
                    // ?????????????????????????????????
                }

                0x0c => {
                    // Client tick end
                }

                _ => {
                    println!("[Play] Unknown Packet ID: 0x{:02X} (Length: {})", packet_id.0, packet_length.0);
                }
            }
        }

        Ok(())
    }

    async fn read_next_packet(&mut self) -> anyhow::Result<(i32, Cursor<Vec<u8>>)> {
        let packet_length: VarInt = self.stream.read_type().await?;

        // Read the number of bytes for the body
        let mut body_buffer = vec![0u8; packet_length.0 as usize];
        self.stream.read_exact(&mut body_buffer).await?;

        // Create a cursor to read the body contents safely
        let mut cursor = Cursor::new(body_buffer);

        // Read the Packet ID from the cursor
        let packet_id: VarInt = cursor.read_type().await?;

        Ok((packet_id.0, cursor))
    }
}

pub async fn send_all_registries(stream: &mut TcpStream) -> anyhow::Result<()> {
    println!("Sending Registry Data...");

    let whitelist: HashSet<&'static str> = HashSet::from([
        "minecraft_damage_type.bin",
        "minecraft_worldgen_biome.bin",
        "minecraft_dimension_type.bin",
        "minecraft_cat_variant.bin",
        "minecraft_chicken_variant.bin",
        "minecraft_cow_variant.bin",
        "minecraft_frog_variant.bin",
        "minecraft_pig_variant.bin",
        "minecraft_wolf_variant.bin",
        "minecraft_wolf_sound_variant.bin",
        "minecraft_painting_variant.bin",
        "minecraft_zombie_nautilus_variant.bin",
        "minecraft_timeline.bin",
        //"minecraft_tags_timeline.bin",
    ]);

    for filename in RegistryData::iter() {
        let name = filename.as_ref();

        if !whitelist.contains(name) {
            continue;
        }

        if name == "packet_tags.bin" {
            continue;
        }

        if let Some(file) = RegistryData::get(name) {
            send_packet(stream, 0x07, &file.data).await?;
            println!("Sent registry: {}", name);
        }
    }

    // Sends the "Update tags - 0x0D" packet
    if let Some(file) = RegistryData::get("packet_tags.bin") {
        println!("Sending Tag Update...");
        send_packet(stream, 0x0D, &file.data).await?;
    }

    Ok(())
}
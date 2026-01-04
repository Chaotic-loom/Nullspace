use std::io::Cursor;
use async_trait::async_trait;
use crate::connection::Connection;
use crate::data_types::BufferReadExt;
use crate::data_types::i_byte::Byte;
use crate::data_types::identifier::Identifier;
use crate::data_types::u_byte::UnsignedByte;
use crate::data_types::var_int::VarInt;
use crate::packets::{Packet, PacketHandler};

pub struct ClientInformationRequestPacket {
    pub locale: String,
    pub view_distance: Byte,
    pub chat_mode: VarInt,
    pub chat_colors: bool,
    pub displayed_skin_parts: UnsignedByte,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: VarInt,
}

impl Packet for ClientInformationRequestPacket {
    fn decode(cursor: &mut Cursor<&[u8]>) -> anyhow::Result<Self> {
        let locale: String = cursor.read_field()?;
        let view_distance: Byte = cursor.read_field()?;
        let chat_mode: VarInt = cursor.read_field()?;
        let chat_colors: bool = cursor.read_field()?;
        let displayed_skin_parts: UnsignedByte = cursor.read_field()?;
        let main_hand: VarInt = cursor.read_field()?;
        let enable_text_filtering: bool = cursor.read_field()?;
        let allow_server_listings: bool = cursor.read_field()?;
        let particle_status: VarInt = cursor.read_field()?;

        Ok(ClientInformationRequestPacket { locale, view_distance, chat_mode, chat_colors, displayed_skin_parts, main_hand, enable_text_filtering, allow_server_listings, particle_status })
    }
}

#[async_trait]
impl PacketHandler for ClientInformationRequestPacket {
    async fn handle(&self, _ctx: &mut Connection) -> anyhow::Result<()> {
        println!("Handling client information request...");

        println!("Locale: {:?}", self.locale);
        println!("View distance: {:?}", self.view_distance);
        println!("Chat mode: {:?}", self.chat_mode);
        println!("Chat colors: {:?}", self.chat_colors);
        println!("Displayed skin parts: {:?}", self.displayed_skin_parts);
        println!("Main hand: {:?}", self.main_hand);
        println!("Enable text filtering: {:?}", self.enable_text_filtering);
        println!("Allow server listings: {:?}", self.allow_server_listings);
        println!("Particle status: {:?}", self.particle_status);

        Ok(())
    }
}
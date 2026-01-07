use tokio::sync::mpsc;
use crate::networking::account::Account;
use crate::networking::connection::{send_packet, NetMessage};
use crate::networking::data_types::PacketWrite;

pub struct Player {
    pub account: Account,
    pub writer: mpsc::Sender<NetMessage>,
}

impl Player {
    pub async fn send_packet<T: PacketWrite>(&mut self, packet_id: i32, packet: T) -> anyhow::Result<()> {
        send_packet(&self.writer, packet_id, packet).await
    }
}
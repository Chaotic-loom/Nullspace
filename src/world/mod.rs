use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use crate::networking::packets::play::keep_alive_response::KeepAliveResponsePacket;
use crate::PlayerList;

pub mod entities;

pub struct World {
    pub players: PlayerList,
    pub tick_count: AtomicU64,
}

impl World {
    pub fn new(player_list: PlayerList) -> Self {
        World {
            players: player_list,
            tick_count: AtomicU64::new(0),
        }
    }

    pub async fn start_tick_loop(&mut self) {
        let mut interval = interval(Duration::from_millis(50));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            interval.tick().await;
            self.tick().await;
        }
    }

    async fn tick(&mut self) {
        let current_tick = self.tick_count.load(Ordering::Relaxed);

        // Keep Alive (Every 15 seconds = 300 ticks)
        if current_tick % 300 == 0 {
            self.broadcast_keep_alive().await;
        }

        self.tick_count.fetch_add(1, Ordering::Relaxed);
    }

    async fn broadcast_keep_alive(&mut self) {
        for mut entry in self.players.iter_mut() {
            let player = entry.value_mut();
            let keep_alive_packet = KeepAliveResponsePacket::new();

            print!("Sending keep alive packet for {}", player.account.username);

            player.send_packet(0x2B, keep_alive_packet).await
                .expect(format!("Error sending keepalive to player ({}, {})", player.account.username.as_str(), player.account.uuid.to_string()).as_str());
        }
    }
}

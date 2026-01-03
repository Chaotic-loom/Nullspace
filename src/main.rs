use tokio::net::{TcpListener};
use crate::connection::{Connection, ConnectionPhase};

mod data_types;
mod packets;
mod connection;

/// # Connection listener
/// Listens for each client connection and handles them until they close.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:25565").await?;
    println!("Server running on 0.0.0.0:25565 (Target: 1.20.1 / Proto: 763)");

    loop {
        let (socket, _) = listener.accept().await?;
        let mut conn = Connection::new(socket, ConnectionPhase::Handshaking);

        tokio::spawn(async move {
            match conn.run().await {
                Ok(_) => {
                    // normal disconnect, no panic
                    println!("Connection closed cleanly at phase {:?}", conn.get_phase());
                }
                Err(e) => {
                    // only unexpected errors logged
                    eprintln!("Connection error at phase {:?}: {:?}", conn.get_phase(), e);
                }
            }
        });
    }
}
use std::sync::Arc;
use tokio::net::TcpListener;
use networking::connection::{Connection, ConnectionPhase};
use networking::packets::configuration::acknowledge_finish_configuration_request::AcknowledgeFinishConfigurationRequestPacket;
use networking::packets::configuration::client_information_request::ClientInformationRequestPacket;
use networking::packets::configuration::known_packs_request::KnownPacksRequestPacket;
use networking::packets::PacketRegistry;
use networking::packets::configuration::plugin_message_configuration_request::PluginMessageConfigurationRequestPacket;
use networking::packets::handshake::HandshakePacket;
use networking::packets::login::login_acknowledged_request::LoginAcknowledgedRequestPacket;
use networking::packets::login::login_start_request::LoginStartRequestPacket;
use networking::packets::play::set_player_position_and_rotation_request::SetPlayerPositionAndRotationRequestPacket;
use networking::packets::play::teleport_confirmation_request::TeleportConfirmationRequestPacket;
use networking::packets::status::ping_request::PingRequestPacket;
use networking::packets::status::status_request::StatusRequestPacket;

mod networking;

/// # Connection listener
/// Listens for each client connection and handles them until they close.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut registry = PacketRegistry::new();
    register_all(&mut registry);

    let registry = Arc::new(registry);

    let listener = TcpListener::bind("0.0.0.0:25565").await?;
    println!("Server running on 0.0.0.0:25565 (Target: 1.20.1 / Proto: 763)");

    loop {
        let (socket, _) = listener.accept().await?;
        let mut conn = Connection::new(socket, ConnectionPhase::Handshaking);

        let registry_ref = registry.clone();

        tokio::spawn(async move {
            match conn.run(&registry_ref).await {
                Ok(_) => {
                    // normal disconnect, no panic
                    println!("Connection closed cleanly at phase {:?}", conn.phase);
                }
                Err(e) => {
                    // only unexpected errors logged
                    eprintln!("Connection error at phase {:?}: {:?}", conn.phase, e);
                }
            }
        });
    }
}

fn register_all(registry: &mut PacketRegistry) {
    // Handshake
    registry.register::<HandshakePacket>(ConnectionPhase::Handshaking, 0x00);

    // Status
    registry.register::<StatusRequestPacket>(ConnectionPhase::Status, 0x00);
    registry.register::<PingRequestPacket>(ConnectionPhase::Status, 0x01);

    // Login
    registry.register::<LoginStartRequestPacket>(ConnectionPhase::Login, 0x00);
    registry.register::<LoginAcknowledgedRequestPacket>(ConnectionPhase::Login, 0x03);

    // Configuration
    registry.register::<ClientInformationRequestPacket>(ConnectionPhase::Configuration, 0x00);
    registry.register::<PluginMessageConfigurationRequestPacket>(ConnectionPhase::Configuration, 0x02);
    registry.register::<AcknowledgeFinishConfigurationRequestPacket>(ConnectionPhase::Configuration, 0x03);
    registry.register::<KnownPacksRequestPacket>(ConnectionPhase::Configuration, 0x07);

    // Play
    registry.register::<TeleportConfirmationRequestPacket>(ConnectionPhase::Play, 0x00);
    registry.register::<SetPlayerPositionAndRotationRequestPacket>(ConnectionPhase::Play, 0x1E);
}
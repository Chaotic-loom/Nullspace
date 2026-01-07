use crate::networking::data_types::{BufferWrite, PacketWrite};
use crate::networking::data_types::i_byte::Byte;
use crate::networking::data_types::identifier::Identifier;
use crate::networking::data_types::position::Position;
use crate::networking::data_types::u_byte::UnsignedByte;
use crate::networking::data_types::var_int::VarInt;

pub struct LoginResponsePacket {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<Identifier>,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: UnsignedByte,
    pub previous_game_mode: Byte,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub death_dimension_name: Option<Identifier>,
    pub death_location: Option<Position>,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
    pub enforces_secure_chat: bool,
}

impl PacketWrite for LoginResponsePacket {
    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.write_type(self.entity_id);
        buf.write_type(self.is_hardcore);
        buf.write_type(self.dimension_names.clone());
        buf.write_type(self.max_players);
        buf.write_type(self.view_distance);
        buf.write_type(self.simulation_distance);
        buf.write_type(self.reduced_debug_info);
        buf.write_type(self.enable_respawn_screen);
        buf.write_type(self.do_limited_crafting);
        buf.write_type(self.dimension_type);
        buf.write_type(self.dimension_name.clone());
        buf.write_type(self.hashed_seed);
        buf.write_type(self.game_mode);
        buf.write_type(self.previous_game_mode);
        buf.write_type(self.is_debug); // TODO: debug world, true deactivates the ability to modify the world, this could be usefully for testing I guess
        buf.write_type(self.is_flat);
        buf.write_type(self.has_death_location);

        if (self.has_death_location) {
            buf.write_type(self.death_dimension_name.clone());
            buf.write_type(self.death_location);
        }

        buf.write_type(self.portal_cooldown);
        buf.write_type(self.sea_level);
        buf.write_type(self.enforces_secure_chat);
    }
}

impl LoginResponsePacket {
    pub fn nullspace() -> LoginResponsePacket {
        LoginResponsePacket {
            entity_id: 2,
            is_hardcore: false,
            dimension_names: vec![
                Identifier::new("minecraft", "overworld")
            ],
            max_players: VarInt(1000),
            view_distance: VarInt(10),
            simulation_distance: VarInt(10),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt(0),
            dimension_name: Identifier::new("minecraft", "overworld"),
            hashed_seed: 0,
            game_mode: UnsignedByte(1),
            previous_game_mode: Byte(0),
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            death_dimension_name: None,
            death_location: None,
            portal_cooldown: VarInt(0),
            sea_level: VarInt(64),
            enforces_secure_chat: true,
        }
    }
}
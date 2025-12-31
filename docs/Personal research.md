C = Cliente
S = Servidor

# C pide MOTD a S
1. C manda un "Handshake - 0x00" con estos datos:
   1. protocol_version: VarInt = _
   2. server_address: String(255) = _
   3. port: Unsigned Short = _
   4. next_state: VarInt = 1
2. C manda un "Handshake - 0x00" vacío confirmando que está listo para recibir datos.
3. S manda un "Handshake - 0x00" con estos datos:
   1. json_response: String(32767)
4. C manda un "Ping - 0x01".
   1. timestamp: Long
5. S manda un "Pong - 0x01". Y manda como timestamp el que le mando C, lo copia.
   1. timestamp: Long
6. C y S cierran la conexión.

# C pide login a S
1. C manda un "Handshake - 0x00" con estos datos:
   1. protocol_version: VarInt = _
   2. server_address: String(255) = _
   3. port: Unsigned Short = _
   4. next_state: VarInt = 2
2. C manda un "Login start - 0x00" con estos datos:
   1. name: String(16)
   2. uuid: UUID
3. S manda "Login Success - 0x02" con estos datos:
   1. profile: GameProfile
4. C manda "Login acknowledged - 0x03" vacío.
5. C manda "Serverbound plugin message configuration - 0x02" con estos datos:
   1. channel: Identifier
   2. data: ?
6. C manda "Client information - 0x00" con estos datos:
   1. locale: String(16)
   2. view_distance: Byte
   3. chat_mode: VarInt (0: enabled, 1: commands only, 2: hidden)
   4. chat_colors: boolean
   5. displayed_skin_parts: Unsigned byte
   6. main_hand: VarInt (0: left, 1: right)
   7. enable_text_filtering: Boolean
   8. allow_server_listings: Boolean
   9. particle_status: VarInt (0: all, 1: decreased, 2: minimal)
7. S manda "Clientbound plugin message configuration - 0x01" con estos datos:
   1. channel: Identifier
   2. data: ?
8. S manda "Feature flags - 0x0C" con estos datos:
   1. feature_flags: Prefixed Array of Identifier
9. S manda "Clientbound known packs - 0x0E" con estos datos:
    1. known_packs: {namespace: String(32767), id: String(32767), version: String(32767)} → Prefixed array
10. C manda "Serverbound known packs - 0x07" con estos datos:
    1. known_packs: {namespace: String(32767), id: String(32767), version: String(32767)} → Prefixed array
11. S manda "Registry data - 0x07" multiples veces con estos datos:
    1. registry_id: Identifier
    2. entries: {entry_id: Identifier, data: Prefixed Optional NBT} → Prefixed array
12. S manda un "Update tags - 0x0D" con estos datos:
    1. tagged_registries: { registry: Identifier, tags: Prefixed Array of Tag } → Prefixed Array
13. S manda "Finish configuration - 0x03" vacío
14. C manda "Acknowledge finish configuration - 0x03" vacío
15. S manda "Login (play) - 0x30" con estos datos:
    1. entity_id: Int
    2. is_hardcore: Boolean
    3. dimension_names: Prefixed array of Identifiers
    4. max_players: VarInt
    5. view_distance: VarInt
    6. simulation_distance: VarInt
    7. reduced_debug_info: Boolean
    8. enable_respawn_screen: Boolean
    9. do_limited_crafting: Boolean
    10. dimension_type: VarInt
    11. dimension name: Identifier
    12. hashed_seed: Long
    13. game_mode: Unsigned Byte
    14. previous_game_mode: Byte
    15. is_debug: Boolean
    16. is_flat: Boolean
    17. has_death_location: Boolean
    18. death_dimension_name: Optional Identifier
    19. death_location: Optional Position
    20. portal_cooldown: VarInt
    21. sea_level: VarInt
    22. enforces_secure_chat: Boolean

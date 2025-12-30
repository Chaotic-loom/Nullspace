C = Cliente
S = Servidor

# Packets
- 0x00: Handshake
- 0x01: Ping & Pong

# C pide MOTD a S
1. C manda un handshake con estos datos:
   1. protocol_version: VarInt = _
   2. server_address: String(255) = _
   3. port: Unsigned Short = _
   4. next_state: VarInt = 1
2. C manda un handshake vacío confirmando que está listo para recibir datos.
3. S manda un handshake con estos datos:
   1. json_response: String(32767)
4. C manda un Ping.
   1. timestamp: Long
5. S manda un Pong. Y manda como timestamp el que le mando C, lo copia.
   1. timestamp: Long
6. C y S cierran la conexión.

# C pide login a S
1. C manda un handshake con estos datos:
   1. protocol_version: VarInt = _
   2. server_address: String(255) = _
   3. port: Unsigned Short = _
   4. next_state: VarInt = 2
2. C manda un handshake con estos datos:
   1. name: String(16)
   2. uuid: UUID
3. S manda 0x02 con estos datos:
   1. profile: GameProfile
4. C manda 0x03 vacío.
5. S manda 0x0E con estos datos:
   1. known_packs: {namespace: String(32767), id: String(32767), version: String(32767)} → Prefixed array
6. C manda 0x07 con estos datos:
   1. known_packs: {namespace: String(32767), id: String(32767), version: String(32767)} → Prefixed array
7. S manda multiples registry data 0x07:
   1. registry_id: Identifier
   2. entries: {entry_id: Identifier, data: Prefixed Optional NBT} → Prefixed array
8. S manda 0x03 para terminar la configuración, esta vacío
9. C manda 0x03 para confirmar la configuración, esta vacío
10. S manda 0x30 con estos datos:
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

GameProfile:
- uuid: UUID
- username: String(16)
- properties: { name: String(64), value: String(32767), signature: Prefixed Optional String(1024) } → Prefixed Array
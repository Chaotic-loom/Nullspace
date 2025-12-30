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
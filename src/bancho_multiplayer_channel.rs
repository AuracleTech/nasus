use crate::{bancho_channel::BanchoChannel, bancho_lobby::BanchoLobby};

/**
 * Represents a multiplayer channel on Bancho.
 */
struct BanchoMultiplayerChannel {
    channel: BanchoChannel,
    lobby: BanchoLobby,
}

use crate::{
    bancho_lobby_player::BanchoLobbyPlayer,
    bancho_lobby_player_score::BanchoLobbyPlayerScore,
    enums::{BanchoLobbyTeamModes, BanchoLobbyWinConditions, BanchoMods, ModeTypes},
};

/**
 * Represents a Bancho multiplayer lobby
 */
pub struct BanchoLobby {
    id: i32,
    name: String,
    users: Vec<BanchoLobbyPlayer>,
    size: i32,
    gamemode: ModeTypes,
    beatmap_id: i32,
    win_condition: BanchoLobbyWinConditions,
    team_mode: BanchoLobbyTeamModes,
    mods: Vec<BanchoMods>,
    freemod: bool,
    playing: bool,
    scores: Vec<BanchoLobbyPlayerScore>,
}

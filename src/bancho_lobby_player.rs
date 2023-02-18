use crate::{
    bancho_lobby::BanchoLobby,
    bancho_lobby_player_score::BanchoLobbyPlayerScore,
    bancho_user::BanchoUser,
    enums::{BanchoLobbyPlayerStates, BanchoLobbyTeams, BanchoMods},
};

pub struct BanchoLobbyPlayer {
    lobby: BanchoLobby,
    user: BanchoUser,
    state: BanchoLobbyPlayerStates,
    hosting: bool,
    team: BanchoLobbyTeams,
    mods: Vec<BanchoMods>,
    store: BanchoLobbyPlayerScore,
}

// TODO reset function

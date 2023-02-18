use crate::bancho_user::BanchoUser;

pub struct BanchoLobbyPlayerScore {
    score: i32,
    pass: bool,
    username: BanchoUser, // Might be a string, check later idk man
}

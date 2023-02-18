use crate::bancho_user::BanchoUser;

/**
 * Bancho incoming message
 */
pub struct BanchoMessage {
    user: BanchoUser,
    message: String,
    homemade: bool,
    action: bool,
}

use crate::bancho_user::BanchoUser;

/**
 * Message received in our private inbox
 */

pub struct PrivateMessage {
    user: BanchoUser,
    recipient: BanchoUser,
    message: String,
}

use crate::bancho_user::BanchoUser;

/**
 * Creates an instance of BanchoChannel
 */
pub struct BanchoChannel {
    name: String,
    topic: String,
    users: Vec<BanchoUser>,
}

// TODO function is joined returns bool if user is in channel

// TODO VERIFY IF ALL THIS IS TRUE
// TODO CM Emitted when a message is received in a BanchoChannel
// TODO Emitted when someone joins this channel
// TODO PART Emitted when someone leaves this channel

use crate::{bancho_channel::BanchoChannel, bancho_user::BanchoUser};

/**
 * A Bancho channel user
 */
pub struct BanchoChannelUser {
    user: BanchoUser,
    channel: BanchoChannel,
    irc_char: char,
    member_mode: String,
}

// TODO IRC letter "v" is "IRC user"
// TODO IRC letter "o" is "Moderator"

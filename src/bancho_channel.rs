use crate::{bancho_lobby::BanchoLobby, bancho_user::BanchoUser};

pub struct BanchoChannelUser {
    user: BanchoUser,
    channel: BanchoChannel,
    irc_char: char,

    // TODO IRC letter "v" is "IRC user"
    // TODO IRC letter "o" is "Moderator"
    member_mode: String,
}

struct BanchoMultiplayerChannel {
    channel: BanchoChannel,
    lobby: BanchoLobby,
}

struct ChannelMessage {
    channel: BanchoChannel,
}

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

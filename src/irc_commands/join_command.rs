use crate::{bancho_channel::BanchoChannel, bancho_user::BanchoUser};

use super::DefaultCommand;

pub struct JoinCommand {
    irc_command: DefaultCommand,
    user: BanchoUser,
    channel: BanchoChannel,
}

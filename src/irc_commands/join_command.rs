use crate::{bancho_channel::BanchoChannel, bancho_user::BanchoUser};

use super::IrcCommand;

struct JoinCommand {
    irc_command: IrcCommand,
    user: BanchoUser,
    channel: BanchoChannel,
}

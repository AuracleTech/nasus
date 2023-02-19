use crate::irc_commands::{
    bad_auth_command::BadAuthCommand, channel_not_found_command::ChannelNotFoundCommand,
    channel_topic_command::ChannelTopicCommand, join_command::JoinCommand,
    message_command::MessageCommand, mode_command::ModeCommand, names_command::NamesCommand,
    part_command::PartCommand, quit_command::QuitCommand,
    user_not_found_command::UserNotFoundCommand, welcome_command::WelcomeCommand,
    whois_channel_command::WhoisChannelCommand, whois_end_command::WhoisEndCommand,
    whois_user_command::WhoisUserCommand,
};

/**
 * BanchoEvent
 */
pub enum BanchoEvent {
    BadAuth(BadAuthCommand),
    ChannelNotFound(ChannelNotFoundCommand),
    ChannelTopic(ChannelTopicCommand),
    Join(JoinCommand),
    Message(MessageCommand),
    Mode(ModeCommand),
    Names(NamesCommand),
    Part(PartCommand),
    Quit(QuitCommand),
    UserNotFound(UserNotFoundCommand),
    Welcome(WelcomeCommand),
    WhoisChannel(WhoisChannelCommand),
    WhoisEnd(WhoisEndCommand),
    WhoisUser(WhoisUserCommand),
}

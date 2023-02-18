mod bad_auth_command;
mod channel_not_found_command;
mod channel_topic_command;
mod join_command;
mod message_command;
mod mode_command;
mod names_command;
mod part_command;
mod quit_command;
mod user_not_found_command;
mod welcome_command;
mod whois_channel_command;
mod whois_end_command;
mod whois_user_command;

// TODO PING COMMAND
/*
    TODO ALL THESE AS COMMANDS
   const ignoredSplits = [
       "312",  // Whois server info (useless on Bancho)
       "333",  // Time when topic was set
       "366",  // End of NAMES reply
       "372",  // MOTD
       "375",  // MOTD Begin
       "376",  // MOTD End
   ];
*/

/**
 * Incoming IRC Command
 */
struct IrcCommand {
    message: String,
    params: Vec<String>,
}

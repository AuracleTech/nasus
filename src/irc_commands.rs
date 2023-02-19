pub mod bad_auth_command;
pub mod channel_not_found_command;
pub mod channel_topic_command;
pub mod join_command;
pub mod message_command;
pub mod mode_command;
pub mod motd_begin_command;
pub mod motd_end_command;
pub mod motd_middle_command;
pub mod names_command;
pub mod part_command;
pub mod quit_command;
pub mod user_not_found_command;
pub mod welcome_command;
pub mod whois_channel_command;
pub mod whois_end_command;
pub mod whois_user_command;

// TODO PING COMMAND
/*
    TODO ALL THESE AS COMMANDS
   const ignoredSplits = [
       "312",  // Whois server info (useless on Bancho)
       "333",  // Time when topic was set

       "366",  // End of NAMES reply
   ];
*/

/**
 * Incoming IRC Command
 */
pub struct DefaultCommand {
    pub message: String,
    pub params: Vec<String>,
    pub homemade: bool,
}

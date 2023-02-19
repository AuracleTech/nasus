#[derive(Clone)]
pub enum OutCommand {
    Ping,
    Pong,
    Login { username: String, irc_token: String },
    SendPM { receiver: String, message: String },
}

impl OutCommand {
    pub fn serialize(&self) -> String {
        match &self {
            OutCommand::Ping => "PING :cho.ppy.sh\r\n".to_string(),
            OutCommand::Pong => "PONG :cho.ppy.sh\r\n".to_string(),
            OutCommand::SendPM { receiver, message } => {
                format!("PRIVMSG {} :{}\r\n", receiver, message)
            }
            OutCommand::Login {
                username,
                irc_token,
            } => format!(
                "PASS {}\r\nNICK {}\r\n",
                irc_token,
                username.replace(" ", "_")
            ),
        }
    }
}

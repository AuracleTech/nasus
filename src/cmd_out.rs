#[derive(Clone, Debug)]
pub enum CmdOut {
    Login { username: String, irc_token: String },
    SendPM { receiver: String, message: String },
    Ping,
    Pong,
}

impl CmdOut {
    pub fn serialize(&self) -> String {
        match &self {
            CmdOut::Ping => "PING :cho.ppy.sh\r\n".to_string(),
            CmdOut::Pong => "PONG :cho.ppy.sh\r\n".to_string(),
            CmdOut::SendPM { receiver, message } => {
                format!("PRIVMSG {} :{}\r\n", receiver, message)
            }
            CmdOut::Login {
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

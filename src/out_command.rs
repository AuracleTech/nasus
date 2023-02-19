use crate::command_kind::CommandKind;

#[derive(Clone)]
pub struct OutCommand {
    pub kind: CommandKind,
}

impl OutCommand {
    pub fn serialize(&self) -> String {
        match &self.kind {
            CommandKind::Pong { line } => line.to_string(),
            CommandKind::SendPM { receiver, message } => {
                format!("PRIVMSG {} :{}\r\n", receiver, message)
            }
            CommandKind::Login {
                username,
                irc_token,
            } => format!(
                "PASS {}\r\nNICK {}\r\n",
                irc_token,
                username.replace(" ", "_")
            ),
            _ => unimplemented!(),
        }
    }
}

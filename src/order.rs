pub struct PrivateMessage {
    pub is_action: bool,
    pub sender: String,
    pub receiver: String,
    pub message: String,
}

pub enum Command {
    ReceivePM(PrivateMessage),
    SendPM(PrivateMessage),
    AuthSuccess(String),
    AuthFailed(String),
    MOTDStart(String),
    MOTDCentral(String),
    MOTDEnd(String),
    Quit(String),
    Ping,
    Unknown,
}

pub struct Order {
    pub command: Command,
    pub line: String,
}

impl Order {
    pub fn parse(line: String, username: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if line.starts_with("PING") {
            return Ok(Self {
                command: Command::Ping,
                line,
            });
        }

        // :Choripan1282!cho@ppy.sh QUIT :replaced (Normal e7g9634-9b39-1b24-2627-911e23746045)\r\n
        // :Tillerino!cho@ppy.sh PRIVMSG Auracle :Unknown command "rr". Type !help if you need help!\r\n
        // :nic2205!cho@ppy.sh QUIT :quit\r\n
        // :CloGamer6!cho@ppy.sh QUIT :ping timeout 180s\r\n
        // :cho.ppy.sh 372 Auracle :- status: https://twitter.com/osustatus\r\n
        // PING cho.ppy.sh\r\n
        let mut split = line.trim_start_matches(':').splitn(2, ':');

        let prefix = if let Some(first) = split.next() {
            first
        } else {
            return Err("sender and code not found".into());
        };

        let suffix = if let Some(second) = split.next() {
            second.trim()
        } else {
            return Err("message not found".into());
        };

        let sender = if prefix.contains('!') {
            prefix.splitn(2, '!').next().unwrap().to_string()
        } else {
            prefix.to_string()
        };

        let suffix_parts = prefix.split(' ').collect::<Vec<&str>>();

        let code = if suffix_parts.len() >= 2 {
            suffix_parts[1]
        } else {
            return Err("code not found".into());
        };

        match code {
            "QUIT" => {
                return Ok(Self {
                    command: Command::Quit(prefix.to_string()),
                    line,
                })
            }
            _ => {}
        }

        let receiver = if suffix_parts.len() >= 3 {
            suffix_parts[2]
        } else {
            return Err("receiver not found".into());
        };

        let message = suffix.to_string();

        let command = match code {
            "464" => Command::AuthFailed(message),
            "001" => Command::AuthSuccess(message),
            "375" => Command::MOTDStart(message),
            "372" => Command::MOTDCentral(message),
            "376" => Command::MOTDEnd(message),
            "PRIVMSG" => {
                let mut pm = PrivateMessage {
                    is_action: false,
                    sender: sender.to_string(),
                    receiver: receiver.to_string(),
                    message: message.to_string(),
                };

                if pm.message.starts_with('\x01') {
                    pm.is_action = true;
                    pm.message.drain(..8);
                }

                if sender == username {
                    Command::SendPM(pm)
                } else {
                    Command::ReceivePM(pm)
                }
            }
            _ => Command::Unknown,
        };

        Ok(Self { command, line })
    }
}

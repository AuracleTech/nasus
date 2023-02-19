#[derive(Clone)]
pub struct InCommand {
    pub line: String,
    pub code: String,
    pub kind: InCommandKind,
}

#[derive(Clone)]
pub enum InCommandKind {
    Ping,
    Pong,
    Quit {
        user: String,
    },
    AuthSuccess,
    AuthFailure,
    MOTDStart,
    MOTDMiddle,
    MOTDEnd,
    UserNotFound,
    ReceivePM {
        sender: String,
        receiver: String,
        message: String,
        action: bool,
    },
}

/*
    TODO ALL THESE AS COMMANDS
   const ignoredSplits = [
       "312",  // Whois server info (useless on Bancho)
       "333",  // Time when topic was set

       "366",  // End of NAMES reply
   ];
*/

impl InCommand {
    // TODO make this return a Result
    pub fn parse(line: String) -> Result<Self, Box<dyn std::error::Error>> {
        if line.starts_with("PING") {
            return Ok(Self {
                line: line.clone(),
                code: "PING".to_string(),
                kind: InCommandKind::Ping,
            });
        }

        let mut split = line.trim_start_matches(':').splitn(2, ':');

        let prefix = if let Some(first) = split.next() {
            first
        } else {
            dbg!(line.clone());
            Err("sender and code not found")?
        };

        let suffix = if let Some(second) = split.next() {
            second.trim()
        } else {
            dbg!(line.clone());
            Err("message not found")?
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
            dbg!(line.clone());
            Err("code not found")?
        };

        match code {
            "PONG" => {
                return Ok(Self {
                    line: line.clone(),
                    code: code.to_string(),
                    kind: InCommandKind::Pong,
                })
            }
            "QUIT" => {
                return Ok(Self {
                    line: line.clone(),
                    code: code.to_string(),
                    kind: InCommandKind::Quit { user: sender },
                })
            }
            _ => {}
        }

        let receiver = if suffix_parts.len() >= 3 {
            suffix_parts[2].to_string()
        } else {
            dbg!(line.clone());
            Err("receiver not found")?
        };

        let mut message = suffix.to_string();

        let data = match code {
            "464" => InCommandKind::AuthFailure,
            "001" => InCommandKind::AuthSuccess,
            "375" => InCommandKind::MOTDStart,
            "372" => InCommandKind::MOTDMiddle,
            "376" => InCommandKind::MOTDEnd,
            "401" => InCommandKind::UserNotFound,
            "PRIVMSG" => {
                let mut action = false;
                if message.starts_with('\x01') {
                    action = true;
                    message.drain(..8);
                }
                InCommandKind::ReceivePM {
                    sender,
                    receiver,
                    message,
                    action,
                }
            }
            _ => panic!("Unknown code from line: {}", line),
        };

        Ok(Self {
            line: line.to_string(),
            code: code.to_string(),
            kind: data,
        })
    }
}

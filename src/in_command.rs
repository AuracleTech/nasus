use super::command_kind::CommandKind;

#[derive(Clone)]
pub struct InCommand {
    pub line: String,
    pub code: String,
    pub kind: CommandKind,
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
                kind: CommandKind::Ping { line },
            });
        }

        let mut split = line.trim_start_matches(':').splitn(2, ':');

        let prefix = if let Some(first) = split.next() {
            first
        } else {
            unimplemented!("No prefix found");
        };

        let suffix = if let Some(second) = split.next() {
            second.trim()
        } else {
            unimplemented!("No suffix found");
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
            unimplemented!("No code found");
        };

        if code == "QUIT" {
            return Ok(Self {
                line: line.clone(),
                code: code.to_string(),
                kind: CommandKind::Quit { user: sender },
            });
        };

        let receiver = if suffix_parts.len() >= 3 {
            suffix_parts[2].to_string()
        } else {
            unimplemented!("No receiver found");
        };

        let mut message = suffix.to_string();

        let data = match code {
            "464" => CommandKind::AuthFailure,
            "001" => CommandKind::AuthSuccess,
            "375" => CommandKind::MOTDStart,
            "372" => CommandKind::MOTDMiddle,
            "376" => CommandKind::MOTDEnd,
            "401" => CommandKind::UserNotFound,
            "PRIVMSG" => {
                let mut action = false;
                if message.starts_with('\x01') {
                    action = true;
                    message.drain(..8);
                }
                // FIX might cause an event loop
                CommandKind::ReceivePM {
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

use crate::NasusResult;

#[derive(Clone, Debug)]
pub enum CmdIn {
    AuthSuccess {
        message: String,
    },
    AuthFailure {
        message: String,
    },
    MOTDStart {
        message: String,
    },
    MOTDMiddle {
        message: String,
    },
    MOTDEnd {
        message: String,
    },
    Quit {
        user: String,
    },
    UserNotFound,
    ReceivePM {
        sender: String,
        receiver: String,
        message: String,
        action: bool,
    },
    Ping,
    Pong,
}


impl CmdIn {
    pub fn parse(line: String) -> NasusResult<Self> {
        if line.starts_with("PING") {
            return Ok(CmdIn::Ping);
        }

        let mut split = line.trim_start_matches(':').splitn(2, ':');

        let prefix = if let Some(first) = split.next() {
            first
        } else {
            Err(format!("prefix not found in line: {}", line))?
        };

        let suffix = if let Some(second) = split.next() {
            second.trim()
        } else {
            Err(format!("suffix not found in line: {}", line))?
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
            Err(format!("code not found in line: {}", line))?
        };

        match code {
            "PONG" => return Ok(CmdIn::Pong),
            "QUIT" => return Ok(CmdIn::Quit { user: sender }),
            _ => {}
        }

        let receiver = if suffix_parts.len() >= 3 {
            suffix_parts[2].to_string()
        } else {
            Err(format!("receiver not found in line: {}", line))?
        };

        let mut message = suffix.to_string();

        let cmd_in = match code {
            "464" => CmdIn::AuthFailure { message },
            "001" => CmdIn::AuthSuccess { message },
            "375" => CmdIn::MOTDStart { message },
            "372" => CmdIn::MOTDMiddle { message },
            "376" => CmdIn::MOTDEnd { message },
            "401" => CmdIn::UserNotFound,
            "PRIVMSG" => {
                let mut action = false;
                if message.starts_with('\x01') {
                    action = true;
                    message.drain(..8);
                }
                CmdIn::ReceivePM {
                    sender,
                    receiver,
                    message,
                    action,
                }
            }
            _ => Err(format!("unknown code in line: {}", line))?,
        };

        Ok(cmd_in)
    }
}

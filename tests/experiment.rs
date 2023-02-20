use nasus::CmdIn;

#[test]
fn parse_ping() {
    let line = "PING cho.ppy.sh\r\n";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::Ping => {}
        _ => panic!("Expected Ping"),
    }
}

#[test]
fn parse_pong() {
    let line = ":cho.ppy.sh PONG :cho.ppy.sh";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::Pong => {}
        _ => panic!("Expected Pong"),
    }
}

#[test]
fn parse_quit() {
    let line = ":AtomicWave!cho@ppy.sh QUIT :quit";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::Quit { user } => {
            assert_eq!(user, "AtomicWave");
        }
        _ => panic!("Expected Pong"),
    }
}

#[test]
fn parse_auth_success() {
    let line = ":cho.ppy.sh 001 Auracle :Welcome to the osu!Bancho.";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::AuthSuccess { message } => {
            assert_eq!(message, "Welcome to the osu!Bancho.");
        }
        _ => panic!("Expected AuthSuccess"),
    }
}

#[test]
fn parse_auth_failure() {
    let line = ":cho.ppy.sh 464 kharnage1 :Bad authentication token.\r\n";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::AuthFailure { message } => {
            assert_eq!(message, "Bad authentication token.");
        }
        _ => panic!("Expected AuthFailure"),
    }
}

#[test]
fn parse_motd_start() {
    let line = ":cho.ppy.sh 375 Auracle :-";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::MOTDStart { message } => {
            assert_eq!(message, "-");
        }
        _ => panic!("Expected MOTDStart"),
    }
}

#[test]
fn parse_motd_central() {
    let line = ":cho.ppy.sh 372 Auracle :-                      __|_o_o_o\\__";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::MOTDMiddle { message } => {
            assert_eq!(message, "-                      __|_o_o_o\\__");
        }
        _ => panic!("Expected MOTDCentral"),
    }
}

#[test]
fn parse_motd_end() {
    let line = ":cho.ppy.sh 376 Auracle :-";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::MOTDEnd { message } => {
            assert_eq!(message, "-");
        }
        _ => panic!("Expected MOTDEnd"),
    }
}

#[test]
fn parse_user_not_found() {
    let line = ":cho.ppy.sh 401 Auracle :No such nick/channel";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::UserNotFound => {}
        _ => panic!("Expected UserNotFound"),
    }
}

#[test]
fn parse_privmsg() {
    let line = ":Tillerino!cho@ppy.sh PRIVMSG Auracle :Unknown command \"rr\". Type !help if you need help!";
    let res = CmdIn::parse(line.to_string());
    let cmd_in = match res {
        Ok(cmd_in) => cmd_in,
        Err(why) => panic!("{}", why),
    };
    match cmd_in {
        CmdIn::ReceivePM {
            sender,
            receiver,
            message,
            action,
        } => {
            assert_eq!(sender, "Tillerino");
            assert_eq!(receiver, "Auracle");
            assert_eq!(
                message,
                "Unknown command \"rr\". Type !help if you need help!"
            );
            assert_eq!(action, false);
        }
        _ => panic!("Expected PrivMsg"),
    }
}

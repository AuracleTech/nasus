use nasus::{InCommand, InCommandKind};

#[test]
fn parse_PING() {
    let line = "PING :cho.ppy.sh\r\n";
    let res = InCommand::parse(line.to_string());
    let in_command = match res {
        Ok(in_command) => in_command,
        Err(why) => panic!("{}", why),
    };
    match in_command.kind {
        InCommandKind::Ping => {}
        _ => panic!("Expected Ping"),
    }
}

#[test]
fn parse_PONG() {
    let line = ":cho.ppy.sh PONG :cho.ppy.sh";
    let res = InCommand::parse(line.to_string());
    let in_command = match res {
        Ok(in_command) => in_command,
        Err(why) => panic!("{}", why),
    };
    match in_command.kind {
        InCommandKind::Pong => {}
        _ => panic!("Expected Pong"),
    }
}

#[test]
fn parse_QUIT() {
    let line = ":AtomicWave!cho@ppy.sh QUIT :quit";
    let res = InCommand::parse(line.to_string());
    let in_command = match res {
        Ok(in_command) => in_command,
        Err(why) => panic!("{}", why),
    };
    match in_command.kind {
        InCommandKind::Quit { user } => {
            assert_eq!(user, "AtomicWave");
        }
        _ => panic!("Expected Pong"),
    }
}
// TODO
/*
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
*/

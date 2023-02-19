#[derive(Clone)]
pub enum CommandKind {
    Ping {
        line: String,
    },
    Pong {
        line: String,
    },
    Quit {
        user: String,
    },
    Login {
        username: String,
        irc_token: String,
    },
    AuthSuccess,
    AuthFailure,
    MOTDStart,
    MOTDMiddle,
    MOTDEnd,
    UserNotFound,
    SendPM {
        receiver: String,
        message: String,
    },
    ReceivePM {
        sender: String,
        receiver: String,
        message: String,
        action: bool,
    },
}

# nasus

nasus is a blazing fast osu BanchoBot handler 📬

#### Example

```rust,ignore
use nasus::{BanchoConfig, CmdIn, CmdOut, Nasus};

let config = BanchoConfig {
    username: "Auracle".to_string(),
    irc_token: "Irc Token".to_string(),
    host: "irc.ppy.sh".to_string(),
    port: 6667,
    bot: false,
};
let mut nasus = match Nasus::new(config).await {
    Ok(nasus) => nasus,
    Err(why) => panic!("Error: {}", why),
};

loop {
    // Read and write commands
    match nasus.work().await {
        Ok(_) => {}
        Err(why) => panic!("Error: {}", why),
    }

    // Handle incoming commands
    match nasus.inputs.pop() {
        Some(cmd_in) => match cmd_in {
            CmdIn::AuthSuccess { message } => println!("{}", message),
            CmdIn::AuthFailure { message } => println!("{}", message),
            CmdIn::ReceivePM {
                sender,
                receiver,
                message,
                action,
            } => {
                println!("{}: {}", sender, message);

                let replies = match action {
                    true => "I see what you did there",
                    false => "I'm not a bot, I swear!",
                };
                let cmd_out = CmdOut::SendPM {
                    receiver: sender,
                    message: replies.to_string(),
                };
                nasus.write_command(cmd_out).await?;
                println!("{}: {}", receiver, replies);
            }
            _ => {}
        },
        None => {}
    };

    // Handle outgoing commands
    match nasus.outputs.pop() {
        Some(cmd_out) => match cmd_out {
            CmdOut::SendPM { receiver, message } => println!("{}: {}", receiver, message),
            _ => {}
        },
        None => {}
    };
}
```

#### Receiving Commands

Commands we receive from BanchoBot are `CmdIn`

```rust,ignore
AuthSuccess { message: String }
AuthFailure { message: String }
MOTDStart { message: String }
MOTDMiddle { message: String }
MOTDEnd { message: String }
Quit { user: String }
UserNotFound
ReceivePM {
    sender: String
    receiver: String
    message: String
    action: bool
}
Ping
Pong
```

Commands we send to BanchoBot are `CmdOut`

```rust,ignore
Login { username: String, irc_token: String }
SendPM { receiver: String, message: String }
Ping
Pong
```

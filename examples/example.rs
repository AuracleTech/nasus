#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use nasus::{BanchoConfig, CmdIn, CmdOut, Nasus};

    let config = BanchoConfig {
        username: dotenv::var("OSU_USERNAME")?,
        irc_token: dotenv::var("OSU_IRC_TOKEN")?,
        host: "irc.ppy.sh".to_string(),
        port: 6667,
        bot: false,
    };
    let mut nasus = Nasus::new(config).await?;

    loop {
        nasus.work().await?;

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

        match nasus.outputs.pop() {
            Some(cmd_out) => match cmd_out {
                CmdOut::SendPM { receiver, message } => println!("{}: {}", receiver, message),
                _ => {}
            },
            None => {}
        };
    }
}

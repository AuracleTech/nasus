use nasus::{BanchoConfig, InCommandKind, Nasus, OutCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = BanchoConfig {
        username: dotenv::var("OSU_USERNAME")?,
        irc_token: dotenv::var("OSU_IRC_TOKEN")?,
        host: "irc.ppy.sh".to_string(),
        port: 6667,
        bot: false,
    };
    let mut nasus = match Nasus::new(config).await {
        Ok(nasus) => nasus,
        Err(why) => panic!("Error: {}", why),
    };

    nasus.send_command(OutCommand::Ping).await?;

    while let Some(in_command) = nasus.next().await? {
        match in_command.kind {
            InCommandKind::AuthSuccess => println!("Auth success"),
            InCommandKind::AuthFailure => println!("Auth failure"),
            InCommandKind::ReceivePM {
                sender,
                receiver,
                message,
                action,
            } => {
                println!("{}: {}", sender, message);

                let reply = match action {
                    true => "I see what you did there",
                    false => "I'm not a bot, I swear!",
                };
                let out_command = OutCommand::SendPM {
                    receiver: sender,
                    message: reply.to_string(),
                };
                nasus.send_command(out_command).await?;
                println!("{}: {}", receiver, reply);
            }
            _ => {}
        }
    }

    Ok(())
}

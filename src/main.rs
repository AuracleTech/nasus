use nasus::{BanchoClient, BanchoConfig, CommandKind, OutCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = dotenv::var("OSU_USERNAME")?;
    let irc_token = dotenv::var("OSU_IRC_TOKEN")?;
    let config = BanchoConfig {
        host: "irc.ppy.sh".to_string(),
        port: 6667,
        bot: false,
    };
    let mut client = match BanchoClient::new(config).await {
        Ok(client) => client,
        Err(why) => panic!("Error: {}", why),
    };

    let login_command = OutCommand {
        kind: CommandKind::Login {
            username,
            irc_token,
        },
    };
    client.send_command(login_command).await?;

    while let Some(in_command) = client.next().await? {
        match in_command.kind {
            CommandKind::AuthSuccess => println!("Auth success"),
            CommandKind::AuthFailure => println!("Auth failure"),
            CommandKind::ReceivePM {
                sender,
                receiver,
                message,
                action,
            } => {
                println!("{}: {}", sender, message);
                if message.starts_with("!r") {
                    let out_command = OutCommand {
                        kind: CommandKind::SendPM {
                            receiver: sender,
                            message: "Ice scream".to_string(),
                        },
                    };
                    client.send_command(out_command).await?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

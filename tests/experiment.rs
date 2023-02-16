use nasus::{Command, Nasus};

#[tokio::test]
async fn env_var_workbench() -> Result<(), Box<dyn std::error::Error>> {
    let username = dotenv::var("OSU_USERNAME").expect("OSU_USERNAME must be set");
    let irc_token = dotenv::var("OSU_IRC_AUTH").expect("OSU_IRC_AUTH must be set");

    let mut nasus = Nasus::new(&username, &irc_token, false).await;

    while let Some(order) = nasus.next().await? {
        match order.command {
            Command::ReceivePM(pm) => {
                println!("{}: {}", pm.sender, pm.message);
                nasus.privmsg(&username, "Auracle moment").await?;
            }
            Command::SendPM(pm) => println!("{}: {}", pm.sender, pm.message),
            Command::AuthSuccess(msg) => println!("{}", msg),
            Command::AuthFailed(msg) => println!("{}", msg),
            _ => {}
        }
    }
    Ok(())
}

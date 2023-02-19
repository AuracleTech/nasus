use nasus::{BanchoClient, BanchoEvent};
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver): (Sender<BanchoEvent>, Receiver<BanchoEvent>) = channel(100);
    let mut client = BanchoClient::new("irc.ppy.sh".to_string(), 6667, false, receiver);

    tokio::spawn(async move {
        while let Some(event) = client.next().await {
            match event {
                BanchoEvent::Message(mc) => {
                    dbg!(mc.irc_command.message);
                }
                _ => {}
            }
        }
    });

    match client.connect().await {
        Ok(_) => {}
        Err(why) => panic!("Error while connecting: {}", why),
    }
    match client.login("Auracle", "Password").await {
        Ok(_) => {}
        Err(why) => panic!("Error while logging in: {}", why),
    }

    Ok(())
}

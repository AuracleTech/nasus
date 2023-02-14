mod nasus;

use colored::Colorize;
use nasus::{BanchoEvent, EventType, Nasus};

#[tokio::main]
async fn main() {
    let username = dotenv::var("OSU_USERNAME").expect("OSU_USERNAME must be set");
    let irc_token = dotenv::var("OSU_IRC_AUTH").expect("OSU_IRC_AUTH must be set");

    let mut nasus = Nasus::new(username, irc_token).await;
    nasus.on(EventType::PrivMsg, |event| {
        println!("{}: {}", event.sender.bold().bright_yellow(), event.message);
    });
    // nasus.on(EventType::PrivMsg, |event| {
    //     println!("{}: {}", event.sender, event.message);
    // });
    // nasus.on(EventType::Quit, |event| {
    //     println!("{} left the chat", event.sender);
    // });
    // nasus
    //     .emit_event(&BanchoEvent {
    //         event_type: EventType::PrivMsg,
    //         sender: "Auracle".to_owned(),
    //         receiver: "Tillerino".to_owned(),
    //         message: "Hello!".to_owned(),
    //     })
    //     .await;
    // nasus
    //     .emit_event(&BanchoEvent {
    //         event_type: EventType::Quit,
    //         sender: "Auracle".to_owned(),
    //         receiver: "Tillerino".to_owned(),
    //         message: "Hello!".to_owned(),
    //     })
    //     .await;
    nasus.listen().await;
}

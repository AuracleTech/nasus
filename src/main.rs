mod nasus;

use colored::Colorize;
use nasus::{Connection, EventBancho, EventDispatcher, EventType};

#[tokio::main]
async fn main() {
    let username = dotenv::var("OSU_USERNAME").expect("OSU_USERNAME must be set");
    let irc_token = dotenv::var("OSU_IRC_AUTH").expect("OSU_IRC_AUTH must be set");

    let mut dispatcher = EventDispatcher::new();
    dispatcher.on(|event| match event.event_type {
        EventType::Error => println!("EventType::Error thrown with buffer: {}", event.message),
        EventType::AuthSuccess => println!("Successfully authenticated as {}", event.sender),
        EventType::AuthFailed => println!("Failed to authenticate as {}", event.sender),
        EventType::PrivMsg => {
            println!(
                "{} {}",
                format!("{}:", event.sender.bright_yellow()),
                event.message
            );
        }
        EventType::PrivAction => {
            println!(
                "{}",
                format!("* {} {} *", event.sender, event.message)
                    .bright_purple()
                    .italic()
            )
        }
        _ => {}
    });
    let mut connection = Connection::new(&mut dispatcher).await;
    // example of how to dispatch an local event
    dispatcher.distribute(EventBancho {
        event_type: EventType::PrivMsg,
        sender: "Peppy".to_string(),
        receiver: "Auracle".to_string(),
        message: "This message is only local".to_string(),
    });

    connection.login(&username, &irc_token).await;
    connection.listen(&mut dispatcher).await;
    // loop {
    //     connection.read().await;
    // }
}

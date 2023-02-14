mod nasus;

use nasus::{BanchoEvent, Connection, EventType};

#[tokio::main]
async fn main() {
    let username = dotenv::var("OSU_USERNAME").expect("OSU_USERNAME must be set");
    let irc_token = dotenv::var("OSU_IRC_AUTH").expect("OSU_IRC_AUTH must be set");

    let mut connection = Connection::new(username, irc_token).await;
    connection.register_event(EventType::PrivMsg, |event| {
        println!("{}: {}", event.sender, event.message);
    });
    // connection.register_event(EventType::PrivMsg, |event| {
    //     println!("{}: {}", event.sender, event.message);
    // });
    // connection.register_event(EventType::Quit, |event| {
    //     println!("{} left the chat", event.sender);
    // });
    // connection.emit_event(&BanchoEvent {
    //     event_type: EventType::PrivMsg,
    //     sender: "Auracle".to_owned(),
    //     receiver: "Tillerino".to_owned(),
    //     message: "Hello!".to_owned(),
    // });
    // connection.emit_event(&BanchoEvent {
    //     event_type: EventType::Quit,
    //     sender: "Auracle".to_owned(),
    //     receiver: "Tillerino".to_owned(),
    //     message: "Hello!".to_owned(),
    // });
    connection.listen().await;
}

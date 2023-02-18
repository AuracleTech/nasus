mod bancho_channel;
mod bancho_channel_user;
mod bancho_client;
mod bancho_event;
mod bancho_lobby;
mod bancho_lobby_player;
mod bancho_lobby_player_score;
mod bancho_lobby_regexes;
mod bancho_message;
mod bancho_mod;
mod bancho_multiplayer_channel;
mod bancho_user;
mod channel_message;
mod enums;
mod event_dispatch;
mod irc_commands;
mod parser;
mod private_message;

use std::collections::HashMap;

pub use event_dispatch::EventDispatch;
use tokio::net::TcpStream;

pub struct BanchoClient {
    port: u16,
    host: String,
    bot: bool,
    events: HashMap<String, Box<dyn FnMut() + Send>>,
}

impl BanchoClient {
    pub fn new(host: String, port: u16, bot: bool) -> Self {
        Self {
            port,
            host,
            bot,
            events: HashMap::new(),
        }
    }

    pub async fn connect(&mut self) -> Result<TcpStream, Box<dyn std::error::Error>> {
        let result = TcpStream::connect(format!("{}:{}", self.host, self.port)).await;
        match result {
            Ok(stream) => Ok(stream),
            Err(why) => Err(Box::new(why)),
        }
    }
}

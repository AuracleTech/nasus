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

pub use bancho_event::BanchoEvent;
pub use event_dispatch::EventDispatch;
use tokio::{io::BufReader, net::TcpStream, sync::mpsc::Receiver};

pub struct BanchoClient {
    port: u16,
    host: String,
    bot: bool,
    receiver: Receiver<BanchoEvent>,
}

impl BanchoClient {
    pub fn new(host: String, port: u16, bot: bool, receiver: Receiver<BanchoEvent>) -> Self {
        Self {
            port,
            host,
            bot,
            receiver,
        }
    }

    pub async fn connect(&mut self) -> Result<TcpStream, Box<dyn std::error::Error>> {
        let result = TcpStream::connect(format!("{}:{}", self.host, self.port)).await;
        match result {
            Ok(stream) => Ok(stream),
            Err(why) => Err(Box::new(why)),
        }
    }

    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let username_format = username.replace(" ", "_");
        Ok(()) //TODO
    }

    pub async fn next(&mut self) -> Option<BanchoEvent> {
        None //TODO
    }
}

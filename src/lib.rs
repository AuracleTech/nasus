mod bancho_channel;
mod bancho_channel_user;
mod bancho_client;
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
mod parser;
mod private_message;

mod command_kind;
mod in_command;
mod out_command;
use std::error::Error;

pub use command_kind::CommandKind;
pub use in_command::InCommand;
pub use out_command::OutCommand;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub struct BanchoConfig {
    pub host: String,
    pub port: u16,
    pub bot: bool,
}

pub struct BanchoClient {
    pub config: BanchoConfig,
    pub reader: BufReader<TcpStream>,
}

impl BanchoClient {
    pub async fn new(config: BanchoConfig) -> Result<Self, Box<dyn Error>> {
        let addr = format!("{}:{}", config.host, config.port);
        let stream = match TcpStream::connect(addr).await {
            Ok(stream) => stream,
            Err(why) => Err(why)?,
        };
        let reader = BufReader::new(stream);
        Ok(Self { config, reader })
    }

    pub async fn next(&mut self) -> Result<Option<InCommand>, Box<dyn std::error::Error>> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;
        if line.is_empty() {
            return Ok(None);
        }
        let in_command = InCommand::parse(line)?;
        self.process(&mut in_command.clone());
        Ok(Some(in_command))
    }

    pub async fn send_command(
        &mut self,
        command: OutCommand,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.reader
            .get_mut()
            .write_all(command.serialize().as_bytes())
            .await?;
        Ok(())
    }

    pub async fn process(&mut self, in_command: &mut InCommand) -> Result<(), Box<dyn Error>> {
        match &mut in_command.kind {
            CommandKind::Ping { line } => {
                let pong_command = OutCommand {
                    kind: CommandKind::Pong { line: line.clone() },
                };
                match self.send_command(pong_command).await {
                    Ok(_) => Ok(()),
                    Err(why) => Err(why),
                }
            }
            _ => Ok(()),
        }
    }
}

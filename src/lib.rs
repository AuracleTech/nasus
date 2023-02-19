mod bancho_channel;
mod bancho_lobby;
mod bancho_mod;
mod bancho_user;
mod enums;
mod parser;

mod in_command;
mod out_command;

pub use in_command::InCommand;
pub use in_command::InCommandKind;
pub use out_command::OutCommand;
use std::error::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub struct BanchoConfig {
    pub host: String,
    pub port: u16,
    pub bot: bool,
    pub username: String,
    pub irc_token: String,
}

pub struct Nasus {
    pub config: BanchoConfig,
    pub reader: BufReader<TcpStream>,
}

impl Nasus {
    pub async fn new(config: BanchoConfig) -> Result<Self, Box<dyn Error>> {
        let addr = format!("{}:{}", config.host, config.port);
        let stream = match TcpStream::connect(addr).await {
            Ok(stream) => stream,
            Err(why) => Err(why)?,
        };
        let reader = BufReader::new(stream);
        let mut nasus = Self { config, reader };
        nasus.login().await?;
        Ok(nasus)
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn Error>> {
        let login_command = OutCommand::Login {
            username: self.config.username.clone(),
            irc_token: self.config.irc_token.clone(),
        };
        self.send_command(login_command).await?;
        Ok(())
    }

    pub async fn next(&mut self) -> Result<Option<InCommand>, Box<dyn std::error::Error>> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;
        if line.is_empty() {
            return Ok(None);
        }
        let in_command = InCommand::parse(line)?;
        self.process(&mut in_command.clone()).await?;
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
            InCommandKind::Ping => match self.send_command(OutCommand::Pong).await {
                Ok(_) => Ok(()),
                Err(why) => Err(why),
            },
            _ => Ok(()),
        }
    }
}

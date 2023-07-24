mod cmd_in;
mod cmd_out;

pub use cmd_in::CmdIn;
pub use cmd_out::CmdOut;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type NasusResult<T> = Result<T, Error>;

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
    pub inputs: Vec<CmdIn>,
    pub outputs: Vec<CmdOut>,
}

impl Nasus {
    pub async fn new(config: BanchoConfig) -> NasusResult<Self> {
        let addr = format!("{}:{}", config.host, config.port);
        let stream = match TcpStream::connect(addr).await {
            Ok(stream) => stream,
            Err(why) => Err(why)?,
        };
        let reader = BufReader::new(stream);
        let mut nasus = Self {
            config,
            reader,
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        nasus.login().await?;
        Ok(nasus)
    }

    pub async fn login(&mut self) -> NasusResult<()> {
        let login_command = CmdOut::Login {
            username: self.config.username.clone(),
            irc_token: self.config.irc_token.clone(),
        };
        self.write_command(login_command).await?;
        Ok(())
    }

    pub async fn work(&mut self) -> NasusResult<()> {
        match self.read().await {
            Ok(_) => {}
            Err(why) => panic!("Error: {}", why),
        }
        match self.write().await {
            Ok(_) => {}
            Err(why) => panic!("Error: {}", why),
        }
        Ok(())
    }

    pub async fn read(&mut self) -> NasusResult<()> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;
        let cmd_in = CmdIn::parse(line)?;
        match cmd_in {
            CmdIn::Ping => match self.write_command(CmdOut::Pong).await {
                Ok(_) => {}
                Err(why) => Err(why)?,
            },
            _ => {}
        }
        self.inputs.push(cmd_in);
        Ok(())
    }

    pub async fn write(&mut self) -> NasusResult<()> {
        if self.outputs.is_empty() {
            return Ok(());
        }
        let cmd_out = self.outputs.remove(0);
        self.write_command(cmd_out).await?;
        Ok(())
    }

    pub async fn write_command(
        &mut self,
        command: CmdOut,
    ) -> NasusResult<()> {
        let res = self
            .reader
            .get_mut()
            .write_all(command.serialize().as_bytes())
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(why) => Err(why)?,
        }
    }
}

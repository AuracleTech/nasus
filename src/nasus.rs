use std::{collections::HashMap, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
};

#[derive(Eq, Hash, PartialEq)]
pub enum EventType {
    AuthSuccess,
    AuthFailed,
    MotdStart,
    Motd,
    MotdEnd,
    Quit,
    PrivMsg,
    Reply,
    Error,
}

pub struct Event {
    pub event_type: EventType,
    pub sender: String,
    pub receiver: String,
    pub message: String,
}

pub struct Connection {
    stream: TcpStream,
    event_handlers: Vec<Box<dyn Fn(&Event) + Send + Sync + 'static>>,
}

impl Connection {
    pub async fn new(username: String, irc_token: String) -> Self {
        Self {
            stream: create_stream(username, irc_token).await,
            event_handlers: Vec::new(),
        }
    }

    pub fn on_event<F>(&mut self, event_type: EventType, handler: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(move |event| {
            if event.event_type == event_type {
                handler(event);
            }
        }));
    }

    pub fn emit(&self, event: &Event) {
        for handler in &self.event_handlers {
            handler(event);
        }
    }
}

async fn create_stream(username: String, irc_token: String) -> TcpStream {
    // create the stream
    const IP_LIST: [&str; 2] = ["irc.ppy.sh", "cho.ppy.sh"];
    const PORT: u16 = 6667;
    const RETRY_INTERVAL_MS: u64 = 5000;

    let mut stream: Option<TcpStream> = None;

    while stream.is_none() {
        for ip in IP_LIST {
            let address = format!("{}:{}", ip, PORT);
            println!("Connecting to {}", address);
            match TcpStream::connect(address).await {
                Ok(s) => {
                    println!("Connection established with {}:{}", ip, PORT);
                    stream = Some(s);
                    break;
                }
                Err(_) => {
                    println!("Connection failed, retrying in {}ms", RETRY_INTERVAL_MS);
                    tokio::time::sleep(std::time::Duration::from_millis(RETRY_INTERVAL_MS)).await;
                    continue;
                }
            }
        }
    }
    // unwrap the stream
    let stream = stream.expect("UNREACHABLE ERROR, PLEASE REPORT THIS TO THE DEVELOPER");
    stream
}

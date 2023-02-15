use peace_performance::{Beatmap, BeatmapExt};
use reqwest;
use std::io::Write;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub enum EventType {
    // when auth succeeds
    AuthSuccess,
    // when auth fails
    AuthFailed,
    // start of the message of the day
    MotdStart,
    // message of the day
    Motd,
    // end of the message of the day
    MotdEnd,
    // when a user leaves the chat
    Quit,
    // private message
    PrivMsg,
    // private message action in private chat example /me is dancing, /np etc...
    PrivAction,
    // when bancho server ping
    Ping,
    // when attempting to connect to bancho
    Connecting,
    // when bancho connection is established
    Connected,
    // when bancho connection fails
    ConnectionFailed,
    // when an error occurs
    Error,
}

#[derive(Debug)]
pub struct EventBancho {
    pub event_type: EventType,
    pub sender: String,
    pub receiver: String,
    pub message: String,
}

pub struct Connection {
    username: String,
    reader: BufReader<TcpStream>,
}

impl Connection {
    pub async fn new(dispatcher: &mut EventDispatcher) -> Self {
        // create the stream
        const IP_LIST: [&str; 2] = ["irc.ppy.sh", "cho.ppy.sh"];
        const PORT: u16 = 6667;
        const RETRY_INTERVAL_MS: u64 = 5000;
        // create the stream
        let mut reader: Option<TcpStream> = None;
        // loop until the stream is successful
        while reader.is_none() {
            for ip in IP_LIST {
                let address = format!("{}:{}", ip, PORT);
                // emit connecting event
                dispatcher.distribute(EventBancho {
                    event_type: EventType::Connecting,
                    sender: String::new(),
                    receiver: String::new(),
                    message: format!("Connecting to {}", address),
                });
                match TcpStream::connect(address).await {
                    Ok(s) => {
                        dispatcher.distribute(EventBancho {
                            event_type: EventType::Connected,
                            sender: String::new(),
                            receiver: String::new(),
                            message: format!("Connection established with {}:{}", ip, PORT),
                        });
                        reader = Some(s);
                        break;
                    }
                    Err(_) => {
                        dispatcher.distribute(EventBancho {
                            event_type: EventType::ConnectionFailed,
                            sender: String::new(),
                            receiver: String::new(),
                            message: format!(
                                "Connection with {}:{} failed, retrying in {}ms",
                                ip, PORT, RETRY_INTERVAL_MS
                            ),
                        });
                        tokio::time::sleep(std::time::Duration::from_millis(RETRY_INTERVAL_MS))
                            .await;
                        continue;
                    }
                }
            }
        }
        // unwrap the stream
        let stream = reader.expect("Failed to unwrap stream");
        Self {
            username: String::new(),
            reader: BufReader::new(stream),
        }
    }

    pub async fn login(&mut self, username: &str, irc_token: &str) {
        // username needs formatting for the irc auth message
        let username_auth_format = username.replace(" ", "_");
        // auth message
        let login = format!("PASS {}\r\nNICK {}\r\n", irc_token, username_auth_format);
        // send auth message
        self.send_server_raw(login).await;
    }

    pub async fn listen(&mut self, dispatcher: &mut EventDispatcher) {
        let mut line = String::new();
        loop {
            // prepare buffer for reading
            line.clear();
            // read lines from the server
            self.reader.read_line(&mut line).await.unwrap();
            // skip empty lines
            if line.is_empty() {
                continue;
            }
            // parse the line
            let event = self.parse_line(line.clone());
            // emit the event
            dispatcher.distribute(event.await);
        }
    }

    pub async fn _send_private_message(&mut self, receiver: &str, message: &str) {
        let message = format!("PRIVMSG {} :{}\r\n", receiver, message);
        self.send_server_raw(message).await;
    }

    /**
     * Send a message to the bancho server
     * @param message The message to send
     */
    pub async fn send_server_raw(&mut self, message: String) {
        // send using reader
        let response = self.reader.write_all(message.as_bytes()).await;
        // check if the response is an error
        match response {
            Ok(_) => (),
            Err(e) => println!("Error sending message: {}", e),
        }
    }

    /**
     * Parses a line received from the bancho server
     * @param line the line to parse
     * @return BanchoEvent the parsed event
     *
     */
    // TODO make Parser its own struct
    async fn parse_line(&mut self, line: String) -> EventBancho {
        // create a new event
        let mut event = EventBancho {
            event_type: EventType::Error,
            sender: String::new(),
            // the person who received the message
            receiver: self.username.clone(),
            message: line.clone(),
        };
        // ping communications are unique and need to be handled separately, they look like this
        // PING cho.ppy.sh\r\n
        if line.starts_with("PING") {
            self.send_server_raw(line.replace("PING", "PONG")).await;
            event.event_type = EventType::Ping;
            return event;
        }
        // most bancho communications are in this format
        // :Tillerino!cho@ppy.sh PRIVMSG Auracle :You really look terrible today you should try sunscream...\r\n
        // the first part is the sender, the second part is the command and the rest depends on the command
        let split_line = line.clone();
        let mut split_line = split_line.split(' ');
        // get the first arg example :Tillerino!cho@ppy.sh
        event.sender = split_line
            .next()
            .expect("Failed to get first arg")
            .to_string();
        // trim the first character ':'
        event.sender = event.sender.trim_start_matches(':').to_string();
        // keep everything before the first '!'
        event.sender = event
            .sender
            .split('!')
            .next()
            .expect("Failed to get first arg")
            .to_string();
        // get the second arg example PRIVMSG
        let command = split_line.next().expect("Failed to get second arg");
        // join the rest of the split line
        let mut message = split_line.clone().collect::<Vec<&str>>().join(" ");
        // trim the message whitespace
        message = message.trim().to_string();
        event.event_type = match command {
            "464" => EventType::AuthFailed,
            "001" => EventType::AuthSuccess,
            "375" => EventType::MotdStart,
            "372" => EventType::Motd,
            "376" => EventType::MotdEnd,
            "QUIT" => EventType::Quit,
            "PRIVMSG" => {
                let mut event_type = EventType::PrivMsg;
                // split the message by spaces
                let mut split_message = message.split(' ');
                // get the first word of the message
                event.receiver = split_message
                    .next()
                    .expect("Failed to get first arg")
                    .to_string();
                // trim the receiver from the message
                event.message = event
                    .message
                    .trim_start_matches(&event.receiver)
                    .to_string();
                // trim a space from the message
                event.message = event.message.trim_start_matches(' ').to_string();
                // remove the semi colon from the message
                event.message = event.message.trim_start_matches(':').to_string();
                // ACTION messages starts with a special character
                let first_char = event
                    .message
                    .chars()
                    .next()
                    .expect("Failed to get first char");
                // :\x01ACTION is listening to [https://osu.ppy.sh/beatmapsets/995092#/2301941 Camellia - Introduction - Akashic Records' Data Collapse]\x01
                if first_char == '\x01' {
                    event_type = EventType::PrivAction;
                    // remove the word ACTION a space and the first special character
                    event.message.drain(..8);
                }
                event_type
            }
            _ => EventType::Error,
        };
        event
    }
}

pub struct EventDispatcher {
    event_handlers: Vec<Box<dyn Fn(&EventBancho) + Send + Sync + 'static>>,
}

impl EventDispatcher {
    /**
     * Create a new event dispatcher
     * @return EventDispatcher the new event dispatcher
     */
    pub fn new() -> Self {
        Self {
            event_handlers: Vec::new(),
        }
    }

    /**
     * Register an event handler
     * @param handler the handler to register
     */
    pub fn on<F>(&mut self, handler: F)
    where
        F: Fn(&EventBancho) + Send + Sync + 'static,
    {
        // add the handler to the list of handlers
        self.event_handlers.push(Box::new(move |event| {
            handler(event);
        }));
    }

    /**
     * Emit an event to all registered event handlers
     * @param event the event to emit
     */
    pub fn distribute(&mut self, event: EventBancho) {
        // for each event registered
        for handler in &self.event_handlers {
            // call the handler
            handler(&event);
        }
    }
}

/**
 * Calculate the performance of a beatmap
 * @param url The beatmap URL
 * @return String containing the performance of the beatmap (95, 97, 98, 99, 100% acc)
 */
pub async fn _calcul_performance(url: &str) -> String {
    // TODO move URL parsing beatmap ID to a function
    let beatmap_set_id = url
        .split('#')
        .next()
        .expect("Failed to get first arg")
        .split('/')
        .last()
        .expect("Failed to get last arg");
    let beatmap_id = url
        .split('#')
        .last()
        .expect("Failed to get last arg")
        .split('/')
        .last()
        .expect("Failed to get last arg");
    // download the map
    let file_name = _download_map(beatmap_id.parse().expect("Failed to parse beatmap_id")).await;
    // open the file
    let file = match tokio::fs::File::open(format!("maps/{}", file_name)).await {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };
    // parse the map asynchronously
    let map = match Beatmap::parse(file).await {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };
    // TODO pass acc list as a parameter
    // accuracy list of 95%, 97%, 98%, 99%, 100%
    let acc = [95.0, 97.0, 98.0, 99.0, 100.0];
    let mut pp = [0.0, 0.0, 0.0, 0.0, 0.0];
    // calculate pp for each acc
    for (i, acc) in acc.iter().enumerate() {
        pp[i] = map.pp().accuracy(*acc).calculate().await.pp();
    }
    // create a string with the pp values
    // TODO create and return a PerformanceResult struct
    let mut result = format!(
        "[https://osu.ppy.sh/beatmapsets/{}#/{} Map] ",
        beatmap_set_id, beatmap_id
    );
    for (i, pp) in pp.iter().enumerate() {
        result.push_str(&format!("{}%: {}pp | ", acc[i], pp.round()));
    }
    // remove the extra separator symbol
    result.pop();
    // return the string
    result
}

/**
 * Download a beatmap using the beatmap id (not the beatmap set id)
 * @param beatmap_id the beatmap id
 * @return String the file path of the .osu file
 */
async fn _download_map(beatmap_id: i32) -> String {
    let url = format!("https://osu.ppy.sh/osu/{}", beatmap_id);
    // use reqwest to get the file
    let response = reqwest::get(&url).await.unwrap();
    // get the file name from the response
    let filename = response
        .url()
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .to_string();
    // make sure a folder called 'maps' exists, if not create it
    std::fs::create_dir_all("maps").expect("Failed to create directory");
    // create a file with the same name in a folder called 'maps'
    // TODO implement a long term data storage
    let mut file =
        std::fs::File::create(format!("maps/{}", filename)).expect("Failed to create file");
    // write the response to the file
    file.write_all(&response.bytes().await.expect("Failed to read bytes"))
        .expect("Failed to write file");
    // return the file name
    // TODO return the full path
    // TEST verify .osu extension is present
    filename
}

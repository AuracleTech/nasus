mod order;
pub use order::{Command, Order};
use peace_performance::{Beatmap, BeatmapExt};
use reqwest;
use std::io::Write;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub struct Nasus {
    reader: BufReader<TcpStream>,
    username: String,
    irc_token: String,
    silent: bool,
}

impl Nasus {
    /**
     * Create a new instance of Nasus
     * @param username the username of the osu account
     * @param irc_token the irc token of the osu account
     * @param silent if true, don't print anything
     */
    pub async fn new(username: &str, irc_token: &str, silent: bool) -> Self {
        let reader = banchobot(silent).await;
        let mut nasus = Self {
            reader,
            username: username.to_string(),
            irc_token: irc_token.to_string(),
            silent,
        };
        nasus.login().await.expect("Failed to login");
        nasus
    }

    async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.print("Authenticating...");
        let username_format = self.username.replace(" ", "_");
        let login_msg = format!("PASS {}\r\nNICK {}\r\n", self.irc_token, username_format);
        self.send_raw(&login_msg).await?;
        Ok(())
    }

    fn print(&self, msg: &str) {
        if !self.silent {
            println!("{}", msg);
        }
    }

    pub async fn next(&mut self) -> Result<Option<Order>, Box<dyn std::error::Error>> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;

        if line.starts_with("PING") {
            let msg = line.replace("PING", "PONG");
            self.send_raw(&msg).await?;
        }

        Ok(Some(Order::parse(line, &self.username)?))
    }

    pub async fn send_raw(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.reader.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    // TODO msg rate limit
    // TODO action msg
    pub async fn privmsg(
        &mut self,
        target: &str,
        msg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.send_raw(&format!("PRIVMSG {} :{}\r\n", target, msg))
            .await?;
        Ok(())
    }
}

/**
 * Connect to BanchoBot and return a buffered reader
 * @param silent if true, don't print anything
 * @return a buffered reader
 */
async fn banchobot(silent: bool) -> BufReader<TcpStream> {
    const BANCHO_IP: [&str; 2] = ["irc.ppy.sh", "cho.ppy.sh"];
    const PORT: u16 = 6667;
    const RETRY_INTERVAL_MS: u64 = 5000;

    let stream: TcpStream;
    let mut ip_index = 0;
    if !silent {
        println!("Connecting to BanchoBot...");
    }
    loop {
        let addr = format!("{}:{}", BANCHO_IP[ip_index], PORT);
        match TcpStream::connect(&addr).await {
            Ok(s) => {
                if !silent {
                    println!("Connected to BanchoBot at {}", addr);
                }
                stream = s;
                break;
            }
            Err(_) => {
                if !silent {
                    println!(
                        "Failed to connect to BanchoBot at {}, retrying in {}ms",
                        addr, RETRY_INTERVAL_MS
                    );
                }
                ip_index += 1;
                if ip_index >= BANCHO_IP.len() {
                    ip_index = 0;
                }
                tokio::time::sleep(std::time::Duration::from_millis(RETRY_INTERVAL_MS)).await;
            }
        }
    }
    // create a buffered reader
    BufReader::new(stream)
}

/**
 * Calculate the performance of a beatmap
 * @param url The beatmap URL
 * @return String containing the performance of the beatmap (95, 97, 98, 99, 100% acc)
 */
pub async fn _calcul_performance(url: &str) -> String {
    // TODO rework functions
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
    // TODO rework functions
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

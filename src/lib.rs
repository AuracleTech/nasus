mod packet;
pub use packet::{Command, Packet};
use peace_performance::{Beatmap, BeatmapExt, PpResult};
use reqwest;
use std::io::Write;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use url::Url;

pub struct Nasus {
    reader: BufReader<TcpStream>,
    username: String,
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
            silent,
        };
        nasus
            .login(irc_token.to_string())
            .await
            .expect("Failed to login");
        nasus
    }

    async fn login(&mut self, irc_token: String) -> Result<(), Box<dyn std::error::Error>> {
        shush_print(self.silent, "Authenticating...");
        let username_format = self.username.replace(" ", "_");
        let login_msg = format!("PASS {}\r\nNICK {}\r\n", irc_token, username_format);
        self.send_raw(&login_msg).await?;
        Ok(())
    }

    pub async fn next(&mut self) -> Result<Option<Packet>, Box<dyn std::error::Error>> {
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;

        if line.starts_with("PING") {
            let msg = line.replace("PING", "PONG");
            self.send_raw(&msg).await?;
        }

        Ok(Some(Packet::parse(line, &self.username)?))
    }

    pub async fn send_raw(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.reader.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    // TODO msg rate limit
    // TODO action msg
    // TODO return Result instead of ()
    pub async fn send_pm(
        &mut self,
        target: &str,
        msg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self
            .send_raw(&format!("PRIVMSG {} :{}\r\n", target, msg))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

fn shush_print(silent: bool, msg: &str) {
    if !silent {
        println!("{}", msg);
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
    shush_print(silent, "Connecting to BanchoBot...");
    loop {
        let addr = format!("{}:{}", BANCHO_IP[ip_index], PORT);
        match TcpStream::connect(&addr).await {
            Ok(s) => {
                shush_print(silent, &format!("Connected to BanchoBot at {}", addr));
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
    BufReader::new(stream)
}

pub struct ParserResult {
    pub beatmap_set_id: i32,
    pub beatmap_id: i32,
    pub url: Url,
}

pub fn get_url_from_text(text: &str) -> Result<ParserResult, Box<dyn std::error::Error>> {
    let parsed_url = if let Some(x) = text.split('[').nth(1) {
        if let Some(y) = x.split(' ').next() {
            y
        } else {
            return Err("Failed to parse url from text".into());
        }
    } else {
        return Err("Failed to parse url from text".into());
    };

    let url = match Url::parse(parsed_url) {
        Ok(url) => {
            if url.host_str() != Some("osu.ppy.sh") {
                return Err("The url is not from osu.ppy.sh".into());
            }
            url
        }
        Err(_) => return Err("Failed to parse url from a string to Url".into()),
    };

    let beatmap_set_id = if let Some(x) = url.to_string().split('#').next() {
        if let Some(y) = x.split('/').last() {
            if let Ok(z) = y.parse::<i32>() {
                z
            } else {
                return Err("Failed to parse beatmap set id from a string to i32".into());
            }
        } else {
            return Err("Failed to parse beatmap set id split by '/'".into());
        }
    } else {
        return Err("Failed to parse beatmap set id split by '#'".into());
    };

    let beatmap_id = if let Some(x) = url.to_string().split('#').last() {
        if let Some(y) = x.split('/').last() {
            if let Ok(z) = y.parse::<i32>() {
                z
            } else {
                return Err("Failed to parse beatmap id from a string to i32".into());
            }
        } else {
            return Err("Failed to parse beatmap id split by '/'".into());
        }
    } else {
        return Err("Failed to parse beatmap id split by '#'".into());
    };

    Ok(ParserResult {
        beatmap_set_id,
        beatmap_id,
        url,
    })
}

// TODO error handling Result on all Option
pub async fn calc_pp_by_acc(osu_file_full_path: &str, accuracy: f32) -> Option<PpResult> {
    if accuracy < 0.0 || accuracy > 100.0 {
        return None;
    }

    let file = match tokio::fs::File::open(osu_file_full_path).await {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let beatmap = match Beatmap::parse(file).await {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    Some(beatmap.pp().accuracy(accuracy).calculate().await)
}

pub async fn download_beatmap_by_id(
    beatmap_id: &i32,
    folder: &str,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let download_url = format!("https://osu.ppy.sh/osu/{}", beatmap_id);
    let response = if let Ok(response) = reqwest::get(&download_url).await {
        response
    } else {
        // TODO verify error handling
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to download beatmap",
        )));
    };

    std::fs::create_dir_all(folder).expect("Failed to create directory");
    let full_path = format!("{}/{}", folder, file_name);
    let mut file = std::fs::File::create(full_path).expect("Failed to create file");
    let bytes = response.bytes().await?; // TODO verify error handling on all lines with ?
    file.write_all(&bytes).expect("Failed to write to file");

    Ok(())
}

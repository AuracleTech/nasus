use dotenv::var;
use peace_performance::{Beatmap, BeatmapExt};
use reqwest;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub const HOST: &str = "irc.ppy.sh"; // TODO backup server address cho.ppy.sh
pub const PORT: u16 = 6667;

#[tokio::main]
async fn main() {
    let user = var("OSU_USERNAME").expect("OSU_USERNAME must be set");
    let pass = var("OSU_IRC_AUTH").expect("OSU_IRC_AUTH must be set");

    // connect to a TcpStream
    let stream =
        TcpStream::connect(format!("{}:{}", HOST, PORT)).expect("Failed to connect to server");

    // create a buffer reader
    let mut reader = BufReader::new(stream);

    // login
    let login = format!("PASS {}\r\nNICK {}\r\n", pass, user.replace(" ", "_"));
    reader.get_mut().write(login.as_bytes()).unwrap();

    // create a buffer to store the lines
    let mut buffer = String::new();
    loop {
        // read lines from the server
        reader.read_line(&mut buffer).expect("Failed to read line");
        // clone the buffer so we can use it later
        let line = buffer.clone();
        // prepare buffer for next read
        buffer.clear();

        // skip empty lines
        if line.is_empty() {
            continue;
        }

        // send PONG to PING to maintain the connection
        // ping messages look like this
        // PING cho.ppy.sh\r\n
        if line.starts_with("PING") {
            reader
                .get_mut()
                .write_all(line.replace("PING", "PONG").as_bytes())
                .expect("Failed to write PONG");
            continue;
        }

        // most messages look like this
        // :Tillerino!cho@ppy.sh PRIVMSG Auracle :You really look terrible today you should try sunscream...\r\n
        // the first part is the sender, the second part is the command and the rest depends on the command
        let mut split_line = line.split(' ');
        let sender = split_line.next().expect("Failed to get first arg");
        let command = split_line.next().expect("Failed to get second arg");

        match command {
            // 464 is sent when the authentification is invalid
            "464" => panic!("Invalid authentification"),
            // 001 is the first message sent after login
            "001" => println!("Logged in as {}", user),
            // 375 is the start of the MOTD
            "375" => continue,
            // 372 is a line of the MOTD
            "372" => continue,
            // 376 is the end of the MOTD
            "376" => continue,
            // PRIVMSG is a private message
            "PRIVMSG" => {
                // remove the first character ':'
                let sender = sender.trim_start_matches(':');
                // remove everything after the first '!' including the '!'
                let sender = sender.split('!').next().expect("Failed to get first arg");
                // Skip my own username
                split_line.next();
                // join the rest of the split line
                let mut msg = split_line.collect::<Vec<&str>>().join(" ");
                // trim the message
                msg = msg.trim().to_string();
                // remove the first character ':'
                msg.remove(0);
                // get the new first character of the message
                let first_char = msg.chars().next().expect("Failed to get first char");
                // match first character of the message
                match first_char {
                    // if it's an action the message looks like this
                    // \x01ACTION is listening to [https://osu.ppy.sh/beatmapsets/57525#/173391 Igorrr - Pavor Nocturnus]\x01
                    '\x01' => {
                        // remove the first 11 characters
                        msg.drain(..11);
                        // remove the last character
                        msg.pop();
                        // get the first word of the message
                        let action = msg.split(' ').next().expect("Failed to get first arg");
                        // get the beatmap URL located after after the first [ and up until a space character
                        let url = msg
                            .split('[')
                            .nth(1)
                            .expect("Failed to get second arg")
                            .split(' ')
                            .next()
                            .expect("Failed to get first arg");
                        // match the action
                        match action {
                            // println!("{} IS LISTENING TO {}", sender, url),
                            "listening" => {
                                reply(calcul_performance(url).await, sender, &mut reader);
                            }
                            "playing" => {
                                reply(calcul_performance(url).await, sender, &mut reader);
                            }
                            "watching" => {
                                reply(calcul_performance(url).await, sender, &mut reader);
                            }
                            "editing" => {
                                reply(calcul_performance(url).await, sender, &mut reader);
                            }
                            _ => println!("UNKNOWN ACTION '{}' FROM THIS LINE '{}'", action, line),
                        }
                    }
                    // if it's a regular message
                    _ => println!("{}: {}", sender, msg),
                }
            }
            // QUIT is a user quitting the server
            "QUIT" => continue,
            _ => println!("UNKNOWN COMMAND '{}' FROM THIS LINE '{}'", command, line),
        }
    }
}

fn reply(msg: String, receiver: &str, reader: &mut BufReader<TcpStream>) {
    // print the reply
    println!("{}", msg);
    // send the reply
    let result = reader
        .get_mut()
        .write_all(format!("PRIVMSG {} :{}\r\n", receiver, msg).as_bytes());
    // check if the write was successful
    match result {
        Ok(_) => (),
        Err(why) => panic!("Failed to write: {}", why),
    }
}

// implement AsyncRead
async fn calcul_performance(url: &str) -> String {
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
    let file_name = download_map(beatmap_id.parse().expect("Failed to parse beatmap_id")).await;
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
    // accuracy list of 95%, 97%, 98%, 99%, 100%
    let acc = [95.0, 97.0, 98.0, 99.0, 100.0];
    let mut pp = [0.0, 0.0, 0.0, 0.0, 0.0];
    // calculate pp for each acc
    for (i, acc) in acc.iter().enumerate() {
        pp[i] = map.pp().accuracy(*acc).calculate().await.pp();
    }
    // create a string with the pp values
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

// function that takes an ID and downloads a file from a url
async fn download_map(beatmap_id: i32) -> String {
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
    filename
}

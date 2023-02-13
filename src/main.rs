use dotenv::var;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub const HOST: &str = "irc.ppy.sh"; // TODO backup server address cho.ppy.sh
pub const PORT: u16 = 6667;

fn main() {
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
                            "listening" => println!("{} IS LISTENING TO {}", sender, url),
                            "playing" => println!("{} IS PLAYING {}", sender, url),
                            "watching" => println!("{} IS WATCHING {}", sender, url),
                            "editing" => println!("{} IS EDITING {}", sender, url),
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

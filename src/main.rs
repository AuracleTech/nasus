use dotenv::var;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub const HOST: &str = "irc.ppy.sh"; // TODO backup server address
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

    // read lines from the server
    let mut line = String::new();
    loop {
        reader.read_line(&mut line).expect("Failed to read line");
        if line.is_empty() {
            continue;
        }

        if line.contains("QUIT") {
            continue;
        }

        if line.starts_with("PING") {
            reader
                .get_mut()
                .write_all(line.replace("PING", "PONG").as_bytes())
                .expect("Failed to write PONG");
        }
        println!("'{}'", line);
        line.clear();
    }
}

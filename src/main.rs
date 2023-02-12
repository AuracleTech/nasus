use dotenv::var;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub const HOST: &str = "irc.ppy.sh"; // TODO backup server address
pub const PORT: u16 = 6667;

struct Connection {
    stream: TcpStream,
    username: String,
}

fn main() {
    let user = var("OSU_USERNAME").expect("OSU_USERNAME must be set");
    let pass = var("OSU_IRC_AUTH").expect("OSU_IRC_AUTH must be set");

    let address = format!("{}:{}", HOST, PORT);
    let mut connection = Connection {
        stream: TcpStream::connect(address).expect("Failed to connect to server"),
        username: user.to_string(),
    };

    let login = format!("PASS {}\r\nNICK {}\r\n", pass, user.replace(" ", "_"));
    connection
        .stream
        .write_all(login.as_bytes())
        .expect("Failed to login");

    let reader = BufReader::new(&connection.stream);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

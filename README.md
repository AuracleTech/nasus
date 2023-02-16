# nasus

nasus is a blazing fast osu BanchoBot handler 📬

#### Example

```rust,ignore
use nasus::{Nasus, Command};
let mut nasus = Nasus::new("Auracle", "OsuIrcToken", false).await;

while let Some(packet) = nasus.next().await? {
    match packet.command {
        Command::AuthSuccess(msg) => println!("{}", msg),
        Command::AuthFailed(msg) => println!("{}", msg),
        Command::SendPM(pm) => println!("{}: {}", pm.sender, pm.message),
        Command::ReceivePM(pm) => {
            println!("{}: {}", pm.sender, pm.message);
            nasus.send_pm(&pm.receiver, "Pepsi Milk").await?;
        }
        _ => {}
    }
}
```

check `tests/experiment.rs` for an example of answering the pp

#### Info

The list of `Command` that we can receive from BanchoBot

```rust,ignore
AuthSuccess(String)       // when we successfully authenticate
AuthFailed(String)        // when we fail to authenticate
MOTDStart(String)         // when we receive the first message of the MOTD
MOTDCentral(String)       // when we receive a middle message of the MOTD
MOTDEnd(String)           // when we receive the last message of the MOTD
SendPM(PrivateMessage)    // when we send a private message
ReceivePM(PrivateMessage) // when someone send us a private message
Quit(String)              // when someone connection is closed
Ping                      // when BanchoBot ping us once every ~3 minutes
Unknown                   // when we receive a message that we don't know how to handle
```

The list of functions that you can use from nasus

```rust,ignore
parse_url_from_np(text) -> Result<ParserResult, Box<dyn std::error::Error>>
calc_pp_by_acc(osu_file_full_path, accuracy) -> Option<PpResult>
download_beatmap_id(beatmap_id, folder, file_name) -> Result<(), Box<dyn std::error::Error>>
```

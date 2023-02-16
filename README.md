# jayce

nasus is a blazing fast osu BanchoBot handler 📬

#### Example

```rust,ignore
use nasus::{Command, Nasus};

#[tokio::test]
async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut nasus = Nasus::new("Auracle", "IrcToken".to_string(), false).await;

    while let Some(packet) = nasus.next().await? {
        match packet.command {
            Command::AuthSuccess(msg) => println!("{}", msg),
            Command::AuthFailed(msg) => println!("{}", msg),
            Command::SendPM(pm) => println!("{}: {}", pm.sender, pm.message),
            Command::ReceivePM(pm) => {
                println!("{}: {}", pm.sender, pm.message);
                nasus
                    .send_pm(&pm.receiver, "Ice scream ice cream")
                    .await?;
            }
            _ => {}
        }
    }
    Ok(())
}

```

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

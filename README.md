# jayce

nasus is a blazing fast osu BanchoBot handler 📬

#### Example

```rust,ignore
use nasus::{Command, Nasus};

#[tokio::test]
async fn test() -> Result<(), Box<dyn std::error::Error>> {
   let mut nasus = Nasus::new("Auracle", "IRCtoken123", false).await;

    while let Some(order) = nasus.next().await? {
        match order.command {
            Command::ReceivePM(pm) => {
                println!("{}: {}", pm.sender, pm.message);
                nasus.privmsg(&username, "Auracle moment").await?;
            }
            Command::SendPM(pm) => println!("{}: {}", pm.sender, pm.message),
            Command::AuthSuccess(msg) => println!("{}", msg),
            Command::AuthFailed(msg) => println!("{}", msg),
            _ => {}
        }
    }
    Ok(())
}

```

#### Info

The list of `Command` types that we can receive from BanchoBot

```rust,ignore
ReceivePM(PrivateMessage) // when someone send us a private message
SendPM(PrivateMessage)    // when we send a private message
AuthSuccess(String)       // when we successfully authenticate
AuthFailed(String)        // when we fail to authenticate
MOTDStart(String)         // when we receive the first message of the MOTD
MOTDCentral(String)       // when we receive a middle message of the MOTD
MOTDEnd(String)           // when we receive the last message of the MOTD
Quit(String)              // when someone connection is closed
Ping                      // when BanchoBot ping us once every ~3 minutes
Unknown                   // when we receive a message that we don't know how to handle
```

use nasus::BanchoClient;

#[tokio::main]
async fn verify() {
    let mut client = BanchoClient::new("irc.ppy.sh".to_string(), 6667, false);
    match client.connect().await {
        Ok(_) => {}
        Err(why) => panic!("Error while connecting: {}", why),
    }
    while let Some(packet) = client.next().await {}
}

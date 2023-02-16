use nasus::{Command, Nasus};

#[tokio::test]
async fn verify_application() -> Result<(), Box<dyn std::error::Error>> {
    let username = dotenv::var("OSU_USERNAME").expect("OSU_USERNAME must be set");
    let irc_token = dotenv::var("OSU_IRC_AUTH").expect("OSU_IRC_AUTH must be set");

    let mut nasus = Nasus::new(&username, irc_token, false).await;

    while let Some(packet) = nasus.next().await? {
        match packet.command {
            Command::AuthSuccess(msg) => println!("{}", msg),
            Command::AuthFailed(msg) => println!("{}", msg),
            Command::SendPM(pm) => println!("{}: {}", pm.sender, pm.message),
            Command::ReceivePM(pm) => {
                println!("{}: {}", pm.sender, pm.message);

                if !pm.action {
                    continue;
                }

                let result = nasus::get_url_from_text(&pm.message);
                let parsed = match result {
                    Ok(parsed) => parsed,
                    Err(_) => continue,
                };

                let file = format!("{}-{}.osu", pm.sender, parsed.beatmap_id);
                let folder = "./maps/";
                let path = format!("{}{}", folder, file);

                if let Err(why) =
                    nasus::download_beatmap_by_id(&parsed.beatmap_id, &folder, &file).await
                {
                    println!("Error while downloading map: {}", why);
                }

                let result = nasus::calc_pp_by_acc(&path, 100.0).await;
                let result = match result {
                    Some(result) => result,
                    None => {
                        println!("Error while calculating pp");
                        continue;
                    }
                };

                let response = format!(
                    "[https://osu.ppy.sh/beatmapsets/{}#/{} {}pp for FC]",
                    parsed.beatmap_set_id,
                    parsed.beatmap_id,
                    result.pp.round()
                );

                nasus.send_pm(&pm.sender, &response).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

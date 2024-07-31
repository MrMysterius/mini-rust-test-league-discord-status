use reqwest::Error;
pub mod events;
pub mod set_activity;
pub mod structs;
use discord_rich_presence::{
    activity::{self, Activity},
    DiscordIpc, DiscordIpcClient,
};
use std::{thread, time};

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("993694995901063198").unwrap();

    client.connect().expect("[DISCORD] CONNECTION FAILED");

    let mut last_event_id: i32 = -1;
    let mut failed_counter = 0;
    let mut events_queue: Vec<structs::Event> = Vec::new();
    let mut change_event_blocker = 0;
    let mut event_string = "".to_string();
    let mut small_text = "none".to_string();
    let mut small_image = "none".to_string();

    loop {
        if change_event_blocker > 0 {
            change_event_blocker -= 1;
        }
        thread::sleep(time::Duration::from_secs(3));

        let player_scores = match getPlayerScores() {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                set_activity::setDefaultActivity(&mut client);
                failed_counter += 1;
                if failed_counter >= 6 {
                    failed_counter = 0;
                    last_event_id = 0;
                }
                continue;
            }
        };

        let event_data = match getEvents() {
            Ok(x) => x,
            Err(err) => {
                // println!("[LEAGUE] FAILED TO GET EVENT DATA");
                println!("{:?}", err);
                failed_counter += 1;
                if failed_counter >= 6 {
                    failed_counter = 0;
                    last_event_id = 0;
                }
                set_activity::setDefaultActivity(&mut client);
                continue;
            }
        };

        failed_counter = 0;

        for event in event_data.Events {
            if event.EventID as i32 > last_event_id {
                last_event_id = event.EventID as i32;
                events_queue.push(event);
            }
        }

        if change_event_blocker == 0 {
            if events_queue.len() > 0 {
                events_queue.rotate_left(1);

                event_string = match events_queue.pop() {
                    Some(event) => {
                        let handled_event = events::handle_event(event);
                        small_image = handled_event.small_image;
                        small_text = handled_event.small_text;
                        change_event_blocker = handled_event.event_display_time;
                        handled_event.event_string
                    }
                    None => {
                        small_text = "none".to_string();
                        small_image = "none".to_string();
                        "".to_string()
                    }
                };
            } else {
                small_text = "none".to_string();
                small_image = "none".to_string();
                event_string = "".to_string();
            }
        }

        println!(
            "{} : {} : {}",
            event_string,
            events_queue.len(),
            last_event_id
        );

        let mut activity = Activity::new();
        let details = format!(
            "{kills}/{deaths}/{assists} ◈ {cs}",
            kills = player_scores.kills,
            deaths = player_scores.deaths,
            assists = player_scores.assists,
            cs = player_scores.creepScore,
        );
        activity = activity.details(&details);
        if event_string.len() > 3 {
            activity = activity.state(&event_string);
        }

        let mut assets = activity::Assets::new();

        assets = assets.large_image("league-logo");
        assets = assets.large_text("Playing League of Legends");
        assets = assets.small_image(small_image.as_str());
        assets = assets.small_text(small_text.as_str());
        // assets = assets.small_image("league-logo");
        // assets = assets.small_text("Playing League of Legends");
        // assets = assets.large_image(small_image.as_str());
        // assets = assets.large_text(small_text.as_str());

        activity = activity.assets(assets);

        match client.set_activity(activity) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
                loop {
                    match client.reconnect() {
                        Ok(_) => {
                            break;
                        }
                        Err(err) => {
                            println!("{:?}", err);
                            thread::sleep(time::Duration::from_secs(10));
                            continue;
                        }
                    }
                }
            }
        }
    }

    // Ok(())
}

#[tokio::main]
async fn getEvents() -> Result<structs::EventData, Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(time::Duration::new(1, 0))
        .build()
        .unwrap();

    let request_url = "https://127.0.0.1:2999/liveclientdata/eventdata".to_string();

    let response = client.get(&request_url).send().await?;
    let json: structs::EventData = response.json().await?;

    Ok(json)
}

#[tokio::main]
async fn getPlayerScores() -> Result<structs::PlayerScores, Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(time::Duration::new(1, 0))
        .build()
        .unwrap();

    let request_url = "https://127.0.0.1:2999/liveclientdata/activeplayername".to_string();

    let response = client.get(&request_url).send().await?;
    let player_name: String = response.json().await?;

    let request_url = "https://127.0.0.1:2999/liveclientdata/playerscores";
    let params = [("summonerName", &player_name)];

    let request_url = reqwest::Url::parse_with_params(request_url, &params).unwrap();

    let response = client.get(request_url).query(&params).send().await?;

    let json: structs::PlayerScores = response.json().await?;

    Ok(json)
}

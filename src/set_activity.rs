use discord_rich_presence::{
    activity::{self, Activity},
    DiscordIpc, DiscordIpcClient,
};
use std::{thread, time};

pub fn setDefaultActivity(client: &mut DiscordIpcClient) {
    let mut activity = Activity::new();
    activity = activity.details("IDLE");

    let mut assets = activity::Assets::new();

    assets = assets.large_image("deno-bi");
    assets = assets.large_text("IDLE");
    assets = assets.small_image("none");
    assets = assets.small_text("none");

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
                        thread::sleep(time::Duration::from_secs(1));
                        continue;
                    }
                }
            }
        }
    }
}

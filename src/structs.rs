use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EventData {
    pub Events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub EventID: u32,
    pub EventName: String,
    pub EventTime: f32,

    #[serde(default)]
    pub Assisters: Vec<String>,

    #[serde(default)]
    pub KillerName: String,

    #[serde(default)]
    pub VictimName: String,

    #[serde(default)]
    pub DragonType: String,

    #[serde(default)]
    pub Stolen: String,

    #[serde(default)]
    pub KillStreak: u32,

    #[serde(default)]
    pub Recipient: String,

    #[serde(default)]
    pub AcingTeam: String,

    #[serde(default)]
    pub Acer: String,
}

impl Default for Event {
    fn default() -> Event {
        Event {
            EventID: Default::default(),
            EventName: Default::default(),
            EventTime: Default::default(),
            Assisters: Default::default(),
            KillerName: Default::default(),
            VictimName: Default::default(),
            DragonType: Default::default(),
            Stolen: Default::default(),
            KillStreak: Default::default(),
            Recipient: Default::default(),
            AcingTeam: Default::default(),
            Acer: Default::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PlayerScores {
    pub assists: u32,
    pub creepScore: u32,
    pub deaths: u32,
    pub kills: u32,
    pub wardScore: f32,
}

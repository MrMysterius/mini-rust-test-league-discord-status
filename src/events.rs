use crate::structs;

pub struct HandledEvent {
    pub small_image: String,
    pub small_text: String,

    pub event_string: String,
    pub event_display_time: u32,
}

pub fn handle_event(event: structs::Event) -> HandledEvent {
    let mut handled_event = HandledEvent {
        small_image: "none".to_string(),
        small_text: "none".to_string(),

        event_string: "⊳ ".to_string(),
        event_display_time: 0,
    };

    println!("{}", event.EventName);

    handled_event.small_text = "none".to_string();
    handled_event.small_image = "none".to_string();

    match event.EventName.as_str() {
        "GameStart" => {
            handled_event.event_string.push_str("Game started");
            handled_event.event_display_time = 5;

            handled_event.small_text = "Game started".to_string();
            handled_event.small_image = "start".to_string();
        }

        "MinionsSpawning" => {
            handled_event.event_string.push_str("Minions are Spawning");
            handled_event.event_display_time = 3;

            handled_event.small_text = "Minions Spawning".to_string();
            handled_event.small_image = "minion".to_string();
        }

        "ChampionKill" => {
            let killer = event.KillerName.as_str();
            let victim = event.VictimName.as_str();
            handled_event.event_display_time = 2;

            handled_event.small_text = format!("{} has kiled {}", killer, victim);
            handled_event.small_image = "kill".to_string();

            handled_event
                .event_string
                .push_str(format!("{} has kiled {}", killer, victim).as_str());
        }

        "FirstBlood" => {
            let recipient = event.Recipient.as_str();
            handled_event.event_display_time = 3;

            handled_event
                .event_string
                .push_str(format!("{} claimed First Blood", recipient).as_str());
            handled_event.small_text = format!("{} claimed First Blood", recipient);
            handled_event.small_image = "first-blood".to_string();
        }

        "DragonKill" => {
            let stolen = event.Stolen.as_str();
            let killer = event.KillerName.as_str();
            let mut dragon_type = event.DragonType.as_str();
            handled_event.event_display_time = 5;

            match dragon_type {
                "Air" => {
                    dragon_type = "Cloud";
                    handled_event.small_image = "cloud_drake".to_string();
                }
                "Earth" => {
                    dragon_type = "Mountain";
                    handled_event.small_image = "mountain_drake".to_string();
                }
                "Fire" => {
                    dragon_type = "Infernal";
                    handled_event.small_image = "infernal_drake".to_string();
                }
                "Water" => {
                    dragon_type = "Ocean";
                    handled_event.small_image = "ocean_drake".to_string();
                }
                "Hextech" => {
                    dragon_type = "Hextech";
                    handled_event.small_image = "hextech_drake".to_string();
                }
                "Elder" => {
                    dragon_type = "Elder";
                    handled_event.small_image = "elder_drake".to_string();
                }
                _ => {}
            }
            handled_event.small_text = format!("{} has slain {} Drake", killer, dragon_type);

            match stolen {
                "False" => {
                    handled_event.event_string.push_str(
                        format!("{} has slain the {} Dragon", killer, dragon_type).as_str(),
                    );
                }
                "True" => {
                    handled_event.event_string.push_str(
                        format!("{} has stolen the {} Dragon", killer, dragon_type).as_str(),
                    );
                }
                _ => {
                    handled_event.event_string = "".to_string();
                }
            }
        }

        "HeraldKill" => {
            let stolen = event.Stolen.as_str();
            let killer = event.KillerName.as_str();
            handled_event.event_display_time = 3;

            match stolen {
                "False" => {
                    handled_event
                        .event_string
                        .push_str(format!("{} has slain the Herald", killer).as_str());
                    handled_event.small_image = "rift_herald".to_string();
                    handled_event.small_text = format!("{} has slain the Herald", killer);
                }
                "True" => {
                    handled_event
                        .event_string
                        .push_str(format!("{} has stolen the Herald", killer).as_str());
                    handled_event.small_image = "rift_herald".to_string();
                    handled_event.small_text = format!("{} has stolen the Herald", killer);
                }
                _ => {}
            }
        }

        "BaronKill" => {
            let stolen = event.Stolen.as_str();
            let killer = event.KillerName.as_str();
            handled_event.event_display_time = 3;

            match stolen {
                "False" => {
                    handled_event
                        .event_string
                        .push_str(format!("{} has slain the Baron", killer).as_str());
                    handled_event.small_image = "baron".to_string();
                    handled_event.small_text = format!("{} has slain the Baron", killer);
                }
                "True" => {
                    handled_event
                        .event_string
                        .push_str(format!("{} has stolen the Baron", killer).as_str());
                    handled_event.small_image = "baron".to_string();
                    handled_event.small_text = format!("{} has stolen the Baron", killer);
                }
                _ => {}
            }
        }

        "FirstBrick" => {
            let killer = event.KillerName.as_str();
            handled_event.event_display_time = 2;

            handled_event
                .event_string
                .push_str(format!("{} has destroyed the First Turret", killer).as_str());
            handled_event.small_image = "turret".to_string();
            handled_event.small_text = format!("{} has destroyed the First Turret", killer);
        }

        "TurretKilled" => {
            let killer = event.KillerName.as_str();
            handled_event.event_display_time = 2;

            handled_event
                .event_string
                .push_str(format!("{} has destroyed a Turret", killer).as_str());
            handled_event.small_image = "turret".to_string();
            handled_event.small_text = format!("{} has destroyed a Turret", killer);
        }

        "InhibKilled" => {
            let killer = event.KillerName.as_str();
            handled_event.event_display_time = 2;

            handled_event
                .event_string
                .push_str(format!("{} has destroyed an Inhibitor", killer).as_str());
            handled_event.small_image = "inhib".to_string();
            handled_event.small_text = format!("{} has destroyed an Inhibitor", killer);
        }

        "InhibRespawningSoon" => {
            handled_event.event_display_time = 1;

            handled_event
                .event_string
                .push_str(format!("An Inhibitor is respawning soon").as_str());
            handled_event.small_image = "inhib".to_string();
            handled_event.small_text = format!("An Inhibitor is respawning soon");
        }

        "InhibRespawned" => {
            handled_event.event_display_time = 2;

            handled_event
                .event_string
                .push_str(format!("An Inhibitor has respawned").as_str());
            handled_event.small_image = "inhib".to_string();
            handled_event.small_text = format!("An Inhibitor has respawned");
        }

        "Ace" => {
            let acing_team = event.AcingTeam.as_str();
            handled_event.event_display_time = 5;

            match acing_team {
                "ORDER" => {
                    handled_event
                        .event_string
                        .push_str(format!("Blue Team Aced").as_str());
                    handled_event.small_image = "ace".to_string();
                    handled_event.small_text = "Blue Team Aced".to_string();
                }
                "CHAOS" => {
                    handled_event
                        .event_string
                        .push_str(format!("Red Team Aced").as_str());
                    handled_event.small_image = "ace".to_string();
                    handled_event.small_text = "Red Team Aced".to_string();
                }
                _ => {}
            }
        }

        _ => {
            handled_event.event_string = "".to_string();
        }
    }

    handled_event
}

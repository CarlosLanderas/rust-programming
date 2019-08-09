#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Serialize};
use serde_json::Serializer;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fmt::Debug;

type RoomId = String;
type RoomExists = Vec<(char, RoomId)>;
type RoomMap = HashMap<RoomId, RoomExists>;


#[derive(Serialize, Deserialize, Debug)]
struct Weapon {
    name: String,
    damage: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    location: String,
    faction: String,
    health: u32,
    weapon: Weapon,
}


fn main() {
    serialize_to_file();
    serialize_to_stdout();
}

fn serialize_to_file() {
    let mut players = vec![];
    players.push(Player {
        location: "Wetlands".to_string(),
        faction: "Horde".to_string(),
        health: 80,
        weapon: Weapon{ name: "Bastard Sword".to_string(), damage: 200},
    });
    players.push(Player {
        location: "Ironforge".to_string(),
        faction: "Alliance".to_string(),
        health: 100,
        weapon: Weapon{ name: "Venom Bow".to_string(), damage: 380},
    });

    let serialized_players = serde_json::to_string_pretty(&players).unwrap();

    let mut file = std::fs::File::create("player.json").unwrap();
    file.write(serialized_players.as_bytes()).unwrap();
    file.flush().unwrap();

    let players: Vec<Player> = serde_json::from_str(&serialized_players).unwrap();
    println!("{:?}", players);
}

fn serialize_to_stdout() {
    let mut map = RoomMap::new();
    map.insert(
        "Cobble Crawl".to_string(),
        vec![('W', "Debris Room".to_string())],
    );
    map.insert(
        "Debris Room".to_string(),
        vec![
            ('E', "Cobble Crawl".to_string()),
            ('W', "Soping Canyon".to_string()),
        ],
    );

    let mut serializer = Serializer::new(io::stdout());

    map.serialize(&mut serializer).unwrap();
}

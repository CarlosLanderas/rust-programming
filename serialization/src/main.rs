#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::io;
use serde::{Serialize, Deserialize};
use serde_json::Serializer;
use std::io::prelude::*;

type RoomId = String;
type RoomExists = Vec<(char, RoomId)>;
type RoomMap = HashMap<RoomId, RoomExists>;

#[derive(Serialize, Deserialize)]
struct Player {
    location: String,
    faction: String,
    health: u32
}

fn main() {

    let mut map = RoomMap::new();
    map.insert("Cobble Crawl".to_string(), vec![('W', "Debris Room".to_string())]);
    map.insert("Debris Room".to_string(), vec![('E', "Cobble Crawl".to_string()), ('W', "Soping Canyon".to_string())]);

    let mut serializer = Serializer::new(io::stdout());

    map.serialize(&mut serializer).unwrap();

    let mut players = vec![];
    players.push(Player{ location: "Wetlands".to_string(), faction: "Horde".to_string(),health :  80});
    players.push(Player{ location: "Ironforge".to_string(), faction: "Alliance".to_string(),health : 100});

    let serialized_players = serde_json::to_string_pretty(&players).unwrap();

    let mut file = std::fs::File::create("player.json").unwrap();
    file.write(serialized_players.as_bytes()).unwrap();
    file.flush().unwrap();

}



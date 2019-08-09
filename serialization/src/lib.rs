use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json;

pub struct City {
    pub name: String,
    pub population: usize
}

pub struct Faction {
    pub name: String,
    pub cities : Vec<City>
}

impl Faction {
    pub fn new(name: String) -> Faction {
        Faction {
            name,
            cities : vec![]
        }
    }

    pub fn add_city(&mut self, name: String, population : usize) -> &mut Faction {
        self.cities.push(City {
            name,
            population
        });

        self
    }

    pub fn as_json(&mut self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }

}


impl Serialize for Faction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {

        let mut state = serializer.serialize_struct("Faction", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("cities", &self.cities)?;
        state.end()
    }
}

impl Serialize for City {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("City", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("population", &self.population)?;
        state.end()
    }
}

use std::fs::File;
use std::time::Duration;
use crate::plant_structures::{Fern, FernType};

pub struct Terrarium {
    ferns: Vec<Fern>
}

impl Terrarium {
    pub fn new() -> Terrarium {
        Terrarium {ferns: vec![]}
    }

    pub fn load(filename: &str) -> Terrarium {
        File::open(filename).unwrap();
        Terrarium{
            ferns: vec![
                Fern::new(FernType::FiddleHead)
            ]
        }
    }

    pub fn fern(&self, index: usize) -> &Fern {
        &self.ferns[index]
    }

    #[allow(unused_variables)]
    pub fn apply_sunlight(&mut self, time: Duration) {
        for f in &mut self.ferns {
            for s in &mut f.stems {
                s.furled = false;
            }
        }
    }
}
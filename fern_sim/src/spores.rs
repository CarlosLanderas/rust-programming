#![allow(dead_code, unused_variables)]

use crate::cells::Cell;

pub struct Spore {
    x: bool
}

pub struct Sporangium {
    x: bool
}

pub fn produce_spore(factory: &mut Sporangium) -> Spore {
    Spore { x: false}
}

fn recombine(parent: &mut Cell) {}
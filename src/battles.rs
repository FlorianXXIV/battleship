use crate::battles::TileType::WATER;
use crate::ship::{SectionType, Ship};

#[derive(Debug)]
pub struct Battlefield {
    battlefield: Vec<Vec<TileType>>,
}

impl Battlefield {
    pub fn get_battlefield(self) -> Vec<Vec<TileType>> {
        self.battlefield
    }
}

#[derive(Debug)]
enum TileType {
    WATER,
    LAND,
    SHIP(SectionType),
}

pub fn init_battle(size:i32, ship: Ship) -> Battlefield {
    let mut out: Vec<Vec<TileType>> = Vec::new();

    for _i in 0..size {
        let mut buf: Vec<TileType> = Vec::new();
        out.push({
            for _j in 0..size {
                buf.push(WATER);
            }
            buf
        });
    }

    let sections = ship.get_sections().unwrap();



    Battlefield { battlefield: out }
}
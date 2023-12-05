use crate::battles::TileType::WATER;
use crate::ship::ShipSection;

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
    SHIP(ShipSection),
}

pub fn init_battle(size:i32) -> Battlefield {
    let mut out: Vec<Vec<TileType>> = Vec::new();

    for i in 0..size {
        let mut buf: Vec<TileType> = Vec::new();
        out.push({
            for j in 0..size {
                buf.push(WATER);
            }
            buf
        });
    }

    Battlefield { battlefield: out }
}
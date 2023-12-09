use crate::battles::TileType::{SHIP, WATER};
use crate::ship::{SectionType, Ship};

#[derive(Debug)]
pub struct Battlefield<'a> {
    battlefield: Vec<Vec<TileType<'a>>>,
}

impl <'a>Battlefield<'_> {
    pub fn get_battlefield(&'a self) -> &Vec<Vec<TileType<'a>>> {
        &self.battlefield
    }

    ///Returns position of tile relative to start
    pub fn get_rel_tile(&self, tile: (usize, usize), tile_face: TileFace, distance: usize) -> (usize, usize){
        match tile_face {
            TileFace::LEFT => {
                (tile.0 - distance, tile.1)
            }
            TileFace::RIGHT => {
                (tile.0 + distance, tile.1)
            }
            TileFace::TopLeft => {
                (tile.0 - distance/2usize, tile.1 - distance/2usize)
            }
            TileFace::TopRight => {
                (tile.0 + distance/2usize, tile.1 - distance/2usize)
            }
            TileFace::BottomLeft => {
                (tile.0 - distance/2usize, tile.1 + distance/2usize)
            }
            TileFace::BottomRight => {
                (tile.0 + distance/2usize, tile.1 + distance/2usize)
            }
        }
    }

    pub fn set_tile_at(&mut self, position: (usize, usize), tile_type: TileType) -> Result<(), &str> {
        if self.battlefield.is_empty() { Err("Battlefield is empty") }
        if self.battlefield.len() < position.0 && self.battlefield[0].len() < position.1 {
            self.battlefield[position.0][position.1] = tile_type;
            Ok(())
        } else {
            Err("Position out of Bounds")
        }
    }
}

#[derive(Debug)]
enum TileType<'a> {
    WATER,
    LAND,
    SHIP(&'a SectionType),
}

pub enum TileFace {
    LEFT,
    RIGHT,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub fn init_battle<'a>(size:usize, ship: &'a Ship) -> Battlefield<'a> {
    let mut out: Vec<Vec<TileType<'a>>> = Vec::new();
    let ship_root = (size/2, size/2);

    for _i in 0..size {
        let mut buf: Vec<TileType> = Vec::new();
        out.push({
            for _j in 0..size {
                buf.push(WATER);
            }
            buf
        });
    }

    let dist = ship.get_center();
    let sections = ship.get_sections();
    for section in sections {
        if section == ship.get_section_at(dist) {
            out[ship_root.0][ship_root.1] = SHIP(section);
        } else {

        }
    }

    Battlefield { battlefield: out }
}
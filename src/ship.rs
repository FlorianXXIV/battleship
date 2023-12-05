use std::io;

#[derive(Debug, Clone)]
pub struct Ship {
    name: String,
    hull: i32,
    crew: i32,
    heavy_ammunition: i32,
    light_ammunition: i32,
    rations: i32,
}

#[derive(Debug)]
pub struct ShipSection {
    section_type: SectionType,
    health: i32,
    ship: Ship,
}

#[derive(Debug)]
pub enum SectionType {
    ENGINE,
    DECK,
    BRIDGE,
}

impl ShipSection {
    pub fn new(section_type: SectionType, ship: Ship) -> ShipSection {
        ShipSection {
            section_type,
            health: 100,
            ship,
        }
    }
}

impl Ship {
    pub fn new() -> Ship{
        Ship {
            name: "".to_string(),
            hull: 0,
            crew: 0,
            heavy_ammunition: 0,
            light_ammunition: 0,
            rations: 0,
        }
    }
}

pub fn init_ship() -> io::Result<Ship> {
    let mut ship_out = Ship {
        name: "".to_string(),
        hull: 100,
        crew: 50,
        heavy_ammunition: 50,
        light_ammunition: 1000,
        rations: 100,
    };
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    ship_out.name = input.trim().to_string();

    Ok(ship_out)
}
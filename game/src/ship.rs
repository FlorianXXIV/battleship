use std::io;

#[derive(Debug, Clone)]
pub struct Ship {
    name: String,
    hull: i32,
    heavy_ammunition: i32,
    light_ammunition: i32,
    rations: i32,
    sections: Vec<SectionType>,
}

/// Section Type:
/// - Types:
///     - ENGINE: Health:i32, Power: i32
///     - DECK: Health:i32, Weapons: Vec<Weapon>
///     - Bridge: Health:i32, Crew:i32, Weapons: Vec<DefenseSystem>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionType {
    ENGINE((i32, i32)),
    DECK((i32, Vec<bool>)),
    BRIDGE((i32, i32, Vec<bool>)),
}

impl Ship {
    pub fn new() -> Ship{
        Ship {
            name: "".to_string(),
            hull: 0,
            heavy_ammunition: 0,
            light_ammunition: 0,
            rations: 0,
            sections: vec![],
        }
    }

    pub fn get_aft(&self) -> usize {
        self.sections.len()-1
    }

    pub fn get_center(&self) -> usize {
        self.sections.len()/2usize
    }

    pub fn get_section_at(&self, index: usize) -> &SectionType {
        &self.sections[index as usize]
    }

    pub fn get_sections(&self) -> &Vec<SectionType> {
        &self.sections
    }
}

pub fn init_ship() -> io::Result<Ship> {
    let mut ship_out = Ship {
        name: "".to_string(),
        hull: 100,
        heavy_ammunition: 50,
        light_ammunition: 1000,
        rations: 100,
        sections: vec![
            SectionType::DECK((100, vec![])),
            SectionType::BRIDGE((100, 50, vec![])),
            SectionType::ENGINE((100, 100))
        ],
    };
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    ship_out.name = input.trim().to_string();

    Ok(ship_out)
}
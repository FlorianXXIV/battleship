use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

use colored::{Color, Colorize};
use json::JsonValue;
use log::{debug, error, info};

use ship::{init_ship, Ship};
use crate::battles::init_battle;

mod battles;
mod ship;

#[derive(Debug)]
struct Town(String);

impl FromStr for Town {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Town(s.to_string()))
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    age: u32,
    hometown: Town,
    captain_id: i64,
}

fn main() {
    env_logger::init();
    info!("Logger initialized");

    menu().expect("Failed to execute Menu");

    info!("Exiting")
}

fn menu() -> Result<(), String> {
    let mut input = String::new();
    println!("\x1B[2J Welcome to Battleship");
    print!("Choose your action\n
    1: NewGame\n
    2: Load\n
    3: Quit\n");

    loop {
        io::stdin().read_line(&mut input).expect("Failed to Read Input");
        debug!("Current input: {:#?}", input.trim());

        match input.trim() {
            "NewGame" | "New" | "1" => {
                input.clear();
                print!("\x1B[2J");
                init_new()
            }
            "LoadGame" | "Load" | "2" => {
                input.clear();
                print!("\x1B[2J");
                load_game()
            }
            "Quit" | "3" | "q" => {
                input.clear();
                break;
            }
            _ => {
                input.clear();
                error!("Input incorrect");
                continue;
            }
        }.expect("Failed to execute menu action");
    }

    Ok(())
}

fn init_new() -> Result<(), String> {
    println!("New Game: ");
    print_txt("data/title.txt", Some(Color::TrueColor {
        r: 0,
        g: 64,
        b: 156,
    })).expect("Failed to print title");
    
    print_txt("lang/en/greetings.txt", None).expect("Failed to print greeting");
    
    let player = init_player().expect("Failed to initialize Player");
    info!("Player Initialized: {:?}", player);
    
    let kern = load_char("kern");
    info!("Loaded kern with: {:?}", kern);
    
    print!("{}", kern["greeting_form_finished"].to_string().white());
    let ship = init_ship().expect("Failed to initialize Ship");
    info!("Ship Initialized: {:?}", ship);

    print_txt("lang/en/greeting2.txt", None).expect("Failed to print greeting.");

    game_loop(ship, player);
    
    Ok(())
}

fn game_loop(ship: Ship, player: Player) {
    let battlefield = init_battle(3);
    println!("{:?}", battlefield)
}

fn load_char(to_load: &str) -> JsonValue {
    let path = "lang/en/characters/".to_owned() + to_load + ".json";
    debug!("{:?}", path);
    json::parse(fs::read_to_string(path).unwrap().as_str())
        .expect("Failed to parse json")
}

fn init_player() -> io::Result<Player>{
    let mut form_input = String::new();
    let mut player_out = Player {
        name: "".to_string(),
        age: 0,
        hometown: Town("".to_string()),
        captain_id: 0,
    };
    let form_json = json::parse(
        fs::read_to_string("data/json/new_game_form.json")
            .expect("Failed to read new_game_form.json")
            .as_str()
    ).expect("Failed to Parse Json");
    debug!("Parsed Object: {:?}", form_json);

    println!("{}", form_json["form_name_txt"]);
    io::stdin().read_line(&mut form_input)?;
    player_out.name = form_input.trim().parse().unwrap();
    form_input.clear();

    println!("{}", form_json["form_age_txt"]);
    io::stdin().read_line(&mut form_input)?;
    player_out.age = form_input.trim().parse().expect("Failed to parse Int");
    form_input.clear();

    println!("{}", form_json["form_town_txt"]);
    io::stdin().read_line(&mut form_input)?;
    player_out.hometown = form_input.trim().parse().expect("Failed to parse Int");
    form_input.clear();

    Ok(player_out)
}

fn print_txt<P: AsRef<Path>>(path: P, color: Option<Color>) -> io::Result<()> {
    if path.as_ref().extension() == Some(OsStr::new("txt")) {
        print!("{}", fs::read_to_string(path)
            .expect("failed to read File")
            .color(match color {
                None => {
                    Color::White
                }
                Some(color) => {
                    color
                }
            })
        );
    } else {
        error!("print_txt tried to read none txt file!")
    }
    Ok(())
}

fn load_game() -> Result<(), String> {
    println!("Loading");
    Ok(())
}

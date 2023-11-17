use std::io;
use log::{debug, error, info, warn};

struct Player {
    name: String,
}

fn main() {
    env_logger::init();
    info!("Logger initialized");

    menu().expect("Failed to execute Menu");

    info!("Exiting")
}

fn menu() -> Result<(), String>{
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
                break
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
    print_title();
    Ok(())
}

fn print_title() {
    print!("\
    #####    ######   ######   ######   #        ######   ######   #    #   #   #####\n\
    #    #   #    #     #         #     #        #        #        #    #   #   #    #\n\
    #    #   #    #     #         #     #        #        #        #    #   #   #    #\n\
    #####    ######     #         #     #        ######   ######   ######   #   ####\n\
    #    #   #    #     #         #     #        #             #   #    #   #   #\n\
    #    #   #    #     #         #     #        #             #   #    #   #   #\n\
    #####    #    #     #         #     ######   ######   ######   #    #   #   #\n\
    \n\
    Greetings Officer, I am Admiral Kern\n\
    Due to your performance in the officer training program, you have been selected to take the place\n\
    of Captain on our newest Overlord-Class Battleship. It will serve as a glorious (...)\n\
    ");
}

fn load_game() -> Result<(), String> {
    println!("Loading");
    Ok(())
}

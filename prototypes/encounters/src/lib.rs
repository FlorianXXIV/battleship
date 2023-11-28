use std::io::{Read, stdin};
use std::io;
use crate::EncounterState::{BREAK, NEXT};

pub enum EncounterState {
    NEXT,
    BREAK,
}

pub fn encounter() -> io::Result<EncounterState>{
    println!{"NEW ENCOUNTER LOL"}
    let mut out_state: EncounterState = NEXT;
    let mut input: String = String::new();
    stdin().read_line(&mut input)?;

    match input.as_str() {
        "quit" | "q" | "exit" => out_state = BREAK,
        _ => println!("{}", input),
    }

    Ok(out_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

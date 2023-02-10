use std::{
    io::{self, Read},
    process,
};

use colored::Colorize;
use serde_json::{to_string_pretty, Value};

fn parse_json(buffer: &String) -> Result<String, serde_json::Error> {
    let parsed_json: Result<Value, serde_json::Error> = serde_json::from_str(&buffer);

    match parsed_json {
        Ok(parsed_json) => to_string_pretty(&parsed_json),
        Err(e) => Err(e),
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    // TODO: Handle invalid buffer
    stdin.read_to_string(&mut buffer)?;

    let parsed_string = parse_json(&buffer);
    match parsed_string {
        Ok(parsed_string) => {
            println!("{}", parsed_string);
        }
        Err(e) => {
            eprintln!("-------");
            eprintln!(
                "[{}] {}",
                "Err".red(),
                "Following error occured during parsing expected json data".bold()
            );
            eprintln!("[{}]\t{:?}", "Err".red(), e);
            eprintln!("[{}] {}", "Err".red(), "please check following data".bold());
            eprintln!("[{}]\n{:?}\n", "Err".red(), buffer);
            eprintln!("-------");

            process::exit(65);
        }
    }

    Ok(())
}

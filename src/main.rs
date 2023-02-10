use std::{
    io::{self, Read},
    process,
};

use colored::Colorize;
use colored_json::{ColorMode, ToColoredJson};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    // TODO: Handle invalid buffer
    stdin.read_to_string(&mut buffer)?;

    let parsed_string = buffer.to_colored_json(ColorMode::On);
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

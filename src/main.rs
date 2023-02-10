use std::{
    io::{self, Read},
    process,
};

use clap::Parser;
use colored::Colorize;
use colored_json::{ColorMode, ToColoredJson};

#[derive(Parser, Debug)]
#[command(author,version, about, long_about = None)]
struct Args {
    /// Disable color mode
    #[arg(short, long, default_value_t = false)]
    no_color: bool,
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    let args = Args::parse();

    let mut color_mode = ColorMode::On;
    if args.no_color == true {
        color_mode = ColorMode::Off;
    }

    // TODO: Handle invalid buffer
    stdin.read_to_string(&mut buffer)?;

    let parsed_string = buffer.to_colored_json(color_mode);
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

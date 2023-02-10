use std::io::{self, Read};

use serde_json::{to_string_pretty, Error, Value};

fn print_parsed_json(json_data: &Value) {
    // TODO: Hande parse errors
    let output = to_string_pretty(&json_data);

    println!("{}", output.unwrap());
}

fn print_parse_error_message(error: Error) {
    eprintln!("{:?}", error)
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    // TODO: Handle invalid buffer
    stdin.read_to_string(&mut buffer)?;

    let parsed_json: Result<_, serde_json::Error> = serde_json::from_str(&buffer);
    match parsed_json {
        Ok(parsed_json) => print_parsed_json(&parsed_json),
        Err(e) => print_parse_error_message(e),
    }

    Ok(())
}

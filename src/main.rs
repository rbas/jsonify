use std::io::{self, Read};

use serde_json::{to_string_pretty, Value};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    // TODO: Handle invalid buffer
    stdin.read_to_string(&mut buffer)?;

    let json_data: Value = serde_json::from_str(&buffer).unwrap();

    // TODO: Hande parse errors
    let output = to_string_pretty(&json_data);

    println!("{}", output.unwrap());

    Ok(())
}

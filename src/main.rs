use dnd_parser::dnd_roll;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let result = args
        .get(1)
        .and_then(|arg| dnd_roll(arg))
        .map(|roll_result| format!("{}", roll_result))
        .unwrap_or_else(|| "Failed to parse roll. Provide argument e.g. '2d20 + 2'".to_string());

    println!("{}", result);
}

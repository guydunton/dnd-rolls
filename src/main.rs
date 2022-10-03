use clap::{Arg, ArgAction, Command};
use dnd_rolls::dnd_roll;

fn main() {
    let matches = Command::new("Dice Roller")
        .about("Roll polyhedral dice from the command line")
        .arg(
            Arg::new("silent")
                .help("Print only the final total")
                .long("silent")
                .short('s')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("INPUT")
                .index(1)
                .required(true)
                .help("The text to parse e.g. 4d8 + 2"),
        )
        .arg_required_else_help(true)
        .get_matches();

    let input = matches.get_one::<String>("INPUT").unwrap();
    let is_silent = matches.get_one::<bool>("silent").unwrap();

    match dnd_roll(input) {
        Some(result) => {
            if !is_silent {
                println!("{}", result);
            } else {
                println!("{}", result.total());
            }
        }
        None => {
            println!("Failed to parse roll. Provide argument e.g. '2d20 + 2'");
        }
    }
}

mod commands;
mod repository;
mod object;
pub mod constant;

use crate::constant::message::INIT_MISSING_COMMAND;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", INIT_MISSING_COMMAND);
        return;
    }

    match args[1].as_str() {
        "init" => commands::init(),
        _ => eprintln!("{}", INIT_MISSING_COMMAND)
    }
}

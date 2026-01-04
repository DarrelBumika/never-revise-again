mod commands;
mod repository;
mod object;
pub mod constant;
mod status;

use crate::constant::message::USAGE_MESSAGE;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", USAGE_MESSAGE);
        return;
    }

    match args[1].as_str() {
        "init" => commands::init(),
        "add" => commands::add(&args[2..]),
        "status" => commands::status(),
        _ => eprintln!("{}", USAGE_MESSAGE)
    }
}

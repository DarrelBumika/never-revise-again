mod constants;
mod handlers;
mod core;
mod entities;
mod utils;

use clap::Parser;

use handlers::repository_handler;
use utils::message_builder;
use entities::cli::{
    Cli,
    Commands
};

fn main() {
    match Cli::try_parse() {
        Ok(cli) => {
            match cli.command {
                Some(Commands::Start) => repository_handler::start(),
                None => message_builder::missing_commands()
            }
        }
        Err(_) => {
            message_builder::invalid_commands()
        }
    }
}

use colored::Colorize;

use crate::constants::message::{
    INVALID_COMMANDS,
    MISSING_COMMANDS,
    START_FAIL_MESSAGE,
    START_SUCCESS_MESSAGE,
    USAGE_MESSAGE
};

pub fn start_success() {
    println!(
        "{}",
        START_SUCCESS_MESSAGE
            .green()
            .bold()
    );
}

pub fn start_fail(err: &str) {
    println!(
        "{}\n\nwith error:\n{}",
        START_FAIL_MESSAGE
            .red()
            .bold(),
        err.to_string()
            .red()
    )
}

pub fn missing_commands() {
    println!(
        "{}{}",
        MISSING_COMMANDS
            .red()
            .bold(),
        USAGE_MESSAGE
            .yellow()
    );
}

pub fn invalid_commands() {
    println!(
        "{}{}",
        INVALID_COMMANDS
            .red()
            .bold(),
        USAGE_MESSAGE
            .yellow()
    );
}
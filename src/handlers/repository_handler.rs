use crate::{
    message_builder,
    core::commands::git_init
};

pub fn start() {
    if let Err(err) = git_init() {
        message_builder::start_fail(&err);
    } else {
        message_builder::start_success()
    }
}
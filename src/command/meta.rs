/*
 * meta_cmd.rs
 * 
 * Handles meta commands (i.e. beginning with '.')
 */

use std::process;
use crate::command::{
    results::*,
};

enum Command {
    EXIT,
    UNKNOWN,
}

pub fn parse(user_input: &str)
{
    cmd_handler(get_cmd_type(user_input));
}

fn get_cmd_type(command: &str) -> Command
{
    match &command[1..] {
        "exit"  =>  Command::EXIT,
        _       =>  Command::UNKNOWN,      
    }
}

fn cmd_handler
(
    cmd: Command
)
-> MetaCommandResult
{
    match cmd {
        Command::EXIT => { 
            process::exit(0); 
        }
        Command::UNKNOWN => { 
            return MetaCommandResult::Unrecognized;
        }
    }
}
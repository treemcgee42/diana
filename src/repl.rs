/*
 * repl.rs
 * 
 * Read Execute Print Loop
 */

use std::io::{self, Write};
use crate::meta_cmd;

enum CommandType {
    META,
    SQL,
}

pub fn read_input()
{
    cursor();

    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_) => {
            let to_parse = line.trim_end();
            parse(to_parse);
        }
        Err(err) => { eprint!("Error: {}", err); }
    }
}

/*******************/
/* Local functions */
/*******************/

fn parse(user_input: &str) -> ()
{
    if user_input.len() == 0 { return; }

    match get_command_type(user_input) {
        CommandType::META => { meta_cmd::parse(user_input); }
        CommandType::SQL => { () }
    }
}

fn get_command_type(command: &str) -> CommandType
{
    match command.chars().next().unwrap() {
        '.' => { return CommandType::META; }
        _ => { return CommandType::SQL; }
    }
}

fn cursor()
{
    print!("DIANA > ");
    io::stdout().flush().unwrap();
}
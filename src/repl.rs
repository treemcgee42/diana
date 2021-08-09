/*
 * repl.rs
 * 
 * Read Execute Print Loop
 */

use std::io::{self, Write};
use crate::data_structs::{
    table::{Table},
};
use crate::command::{
    statement,
    meta,
};

enum CommandType {
    META,
    SQL,
}

pub fn read_input(table: &mut Table)
{
    cursor();

    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_) => {
            let to_parse = line.trim_end();
            parse(to_parse, table);
        }
        Err(err) => { eprint!("Error: {}", err); }
    }
}

/*******************/
/* Local functions */
/*******************/

fn parse(user_input: &str, table: &mut Table) -> ()
{
    if user_input.len() == 0 { return; }

    match get_command_type(user_input) {
        CommandType::META => { meta::parse(user_input); }
        CommandType::SQL => { statement::handle(user_input, table); }
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
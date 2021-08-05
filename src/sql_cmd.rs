/*
 * sql_cmd.rs
 * 
 * Handles SQL commands
 */

pub enum SQLCommand {
    Insert(String),
    Unknown(String),
}

impl SQLCommand {
    pub fn new(user_input: String) -> SQLCommand
    {
        let split_input = user_input.split(" ").collect::<Vec<&str>>();

        match split_input[0] {
            "insert" => { SQLCommand::Insert(user_input) }
            _ => { SQLCommand::Unknown(user_input) }
        }
    }
}
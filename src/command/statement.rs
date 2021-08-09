/*
 * statement.rs
 * 
 * Regular (SQL) statements
 */

use crate::data_structs::{
    *, 
    row::*, 
    table::*,
    constants::*,
};

use crate::command::{
    results::*,
};

/****************/
/* Global types */
/****************/

pub struct Statement {
    stype: StatementType,
    row_to_insert: Row,
}

pub enum StatementType {
    Insert,
    Select,
}

/*******************/
/* Implementations */
/*******************/

impl Statement {
    pub fn new() 
    -> Statement
    {
        Statement {
            stype: StatementType::Select,
            row_to_insert: Row::new(),
        }
    }
}

/********************/
/* Global functions */
/********************/

pub fn handle(user_input: &str, table: &mut Table)
{
    let mut statement = Statement::new();
    
    match prepare_statement(user_input, &mut statement) {
        PrepareResult::Success => (),
        PrepareResult::SyntaxError => {
            println!("Your syntax is off, I couldn't parse '{}'.", user_input);
            return;
        }
        PrepareResult::Unrecognized => {
            println!("Unrecognized keyword at start of '{}'.", user_input);
            return;
        }
    }

    execute_statement(&statement, table);
    println!("Executed.");
}

/*******************/
/* Local functions */
/*******************/

fn prepare_statement(user_input: &str, statement: &mut Statement) 
-> PrepareResult
{
    let split_input = user_input.split(" ").collect::<Vec<&str>>();

    match split_input[0] {
        "insert" => {
            statement.stype = StatementType::Insert;
            return parse_row(user_input, statement);
        }
        "select" => {
            statement.stype = StatementType::Select;
            return PrepareResult::Success;
        }
        _ => { return PrepareResult::Unrecognized; }
    }
}

fn execute_statement(statement: &Statement, table: &mut Table)
-> ExecuteResult
{
    match statement.stype {
        StatementType::Insert => {
            return execute_insert(statement, table);
        }
        StatementType::Select => {
            return execute_select(statement, table);
        }
        _ => {
            return println!("(none)");
        }
    }
}

fn execute_insert(statement: &Statement, table: &mut Table)
-> ExecuteResult
{
    if table.num_rows >= TABLE_MAX_ROWS { return ExecuteResult::TableFull; }

    let row_serialized = Serializable::serialize( &statement.row_to_insert );

    let (page, byte_offset) = table.row_slot(table.num_rows);
    page.insert(row_serialized, byte_offset, ROW_SIZE);
    table.num_rows += 1;

    return ExecuteResult::Success;
}

fn execute_select(statement: &Statement, table: &mut Table)
-> ExecuteResult
{
    for i in 0..table.num_rows {
        let (page, byte_offset) = table.row_slot(i);
        let a = &page.bytes[byte_offset..(byte_offset+ROW_SIZE)];

        println!("{}", Row::deserialize(a));
    }

    return ExecuteResult::Success;
}

fn parse_row(user_input: &str, statement: &mut Statement)
-> PrepareResult
{
    let split_input = user_input.split(" ").collect::<Vec<&str>>();

    let id: u32;
    let id_ = split_input[1].parse::<u32>();
    match id_ {
        Ok(i) => { id = i; }
        Err(_) => { return PrepareResult::SyntaxError; }
    }

    let username = String::from(split_input[2]);
    let email = String::from(split_input[3]);

    statement.row_to_insert = Row::create(id, username, email);
    return PrepareResult::Success;
}
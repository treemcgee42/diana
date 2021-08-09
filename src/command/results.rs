/*
 * results.rs
 * 
 * Various result enums
 */

pub enum PrepareResult {
    Success,
    SyntaxError,
    Unrecognized,
}

pub enum MetaCommandResult {
    Success,
    Unrecognized,
}

pub enum ExecuteResult {
    Success,
    TableFull,
}
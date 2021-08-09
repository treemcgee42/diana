mod repl;
mod command;
mod data_structs;

use crate::data_structs::{
    table::*
};

fn main() {
    let mut table = Table::new();

    loop {
        repl::read_input(&mut table);
    }
}

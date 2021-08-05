mod repl;
mod meta_cmd;
mod sql_cmd;

fn main() {
    loop {
        repl::read_input();
    }
}

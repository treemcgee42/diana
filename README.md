# diana

A SQLite clone based on the C tutorial [here](https://cstack.github.io/db_tutorial/). So the purpose of this is not only to follow the tutorial there 
and understand how SQLite works, but also an excercise in porting programming techniques in C to Rust. In many instances Rust provides "safe" alternatives,
but the low-level nature of a database demands careful usage of "unsafe" Rust in certain places.

For an instance of this, see the `DESIGN.md` in `src/data_structs/`.

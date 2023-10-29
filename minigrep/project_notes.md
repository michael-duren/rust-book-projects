# Project for learning

To enable `minigrep` to read values of cli args we pass to it,
we'll need the `std::eng::args fn in Rusts std lib

## The args fn and invalid unicode

Note that std::env::args will panic if any argument 
contains invalid Unicode. If your program needs to accept arguments 
containing invalid Unicode, use std::env::args_os instead. 
That function returns an iterator that produces OsString values instead of String values. 
We’ve chosen to use std::env::args here for simplicity because OsString values differ per 
platform and are more complex to work with than String values.

We use the `collect` to turn the iterator into a vector containing all the values produced 
by the iterator. `colllect` is one fn you do often need to annotate because Rust isn't able to
infer the kind of collection you want.

## Separation of Concerns for Binary Projects

The organizational problem of allocating responsibility for multiple tasks
to the `main` fn is common to many binary projects.
As a result, the Rust community has developed guidelines for splitting the separate
concerns of a binary program when `main` starts getting large.

- Split your program into a `main.rs` file and a `lib.rs` file and move your program's
logic to `lib.rs`
- As long as your command line parsing logic is small, it can remain in `main.rs`
- When the clie parsing logic starts getting complicated, extract it from `main.rs` and 
move it to `lib.rs`

The responsibilities that remain in the main fn after this process should be limited to:
- Calling the cli parsing logic with the arg values
- Setting up any other config
- Calling a `run` fn in `lib.rs`
- Handling the error if `run` returns an error

LEFT OFF AT PG 370 in Rust e book

use std::env;

use rc_todo::{help, Todo};

fn main() {
    let todo = Todo::new().expect("Created todo instance failed");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "rm" => todo.remove(&args[2..]),
            "done" => todo.done(&args[2..]),
            "sort" => todo.sort(),
            "rest" => todo.reset(),
            "help" | "--help" | "-h" | _ => help(),
        }
    } else {
        todo.list();
    }
    println!("Hello, world!");
}

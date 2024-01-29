use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::process::exit;
use std::{env, io};

pub struct Todo {
    pub todo_list: Vec<String>,
    pub todo_path: String,
    pub todo_bk: String,
    pub is_backup: bool,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        let todo_path = match env::var("TODO") {
            Ok(t) => t,
            Err(_) => {
                let home = env::var("HOME").unwrap();
                let default_path = format!("{}/TODO", &home);
                match Path::new(&default_path).exists() {
                    true => default_path,
                    false => format!("{}/.todo", home),
                }
            }
        };

        let todo_bk = match env::var("TODO_BAK") {
            Ok(t) => t,
            Err(_) => String::from("/tmp/todo.bak"),
        };

        let is_backup = env::var("TODO_NO_BACKUP").is_ok();

        let todo_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect("Couldn't open the file");

        let mut buffer_reader = BufReader::new(&todo_file);
        let mut contents = String::new();
        buffer_reader.read_to_string(&mut contents).unwrap();

        let todo_list = contents.lines().map(str::to_string).collect();

        Ok(Self {
            todo_list,
            todo_path,
            todo_bk,
            is_backup,
        })
    }

    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least one argument");
            exit(1);
        }
        // println!("{:?}", args);
        let todo_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todo_file);
        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            let line = format!("[ ] {}\n", arg);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprint!("todo remove takes at least 1 argument");
            exit(1);
        }
        let todo_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Could't open todo file");
        let mut buffer = BufWriter::new(todo_file);

        for (index, line) in self.todo_list.iter().enumerate() {
            if args.contains(&"done".to_string()) && &line[..4] == "[*]" {
                continue;
            }
            if args.contains(&(index + 1).to_string()) {
                continue;
            }
            let line = format!("{}\n", line);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data")
        }
    }

    pub fn done(&self, _args: &[String]) {
        //
    }

    pub fn reset(&self) {
        //
    }

    pub fn sort(&self) {
        //
    }

    pub fn list(&self) {
        let stdout = io::stdout();
        let mut write = BufWriter::new(stdout);
        let mut data = String::new();
        for (index, task) in self.todo_list.iter().enumerate() {
            // task eg: [ ] this is a todo demo
            if task.len() > 5 {
                let number = (index + 1).to_string();
                let symbol = &task[..4];
                let task = &task[4..];
                if symbol == "[*]" {
                    data = format!("{} {}\n", number, task);
                } else if symbol == "[ ]" {
                    data = format!("{} {}\n", number, task);
                }
                write
                    .write_all(data.as_bytes())
                    .expect("Failed to write to stdout")
            }
        }
    }
}

use std::fs::{self, OpenOptions};
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
        let home = env::var("HOME").unwrap();
        let default_path = format!("{}/TODO", &home);
        let todo_path = match env::var("TODO") {
            Ok(t) => t,
            Err(_) => match Path::new(&default_path).exists() {
                true => default_path,
                false => format!("{}/.todo", home),
            },
        };

        let todo_bk = match env::var("TODO_BAK") {
            Ok(t) => t,
            Err(_) => format!("{}/todo.bak", home),
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
            eprintln!("todo remove takes at least 1 argument");
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

    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            exit(1);
        }
        let todo_file = OpenOptions::new()
            .write(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");
        let mut buffer = BufWriter::new(todo_file);
        for (index, line) in self.todo_list.iter().enumerate() {
            if line.len() > 5 {
                if args.contains(&(index + 1).to_string()) {
                    if &line[..4] == "[ ] " {
                        let line = format!("[*] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("Unable to write data");
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {} \n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("Unable to write data");
                    }
                } else if &line[..4] == "[ ] " || &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    buffer
                        .write_all(line.as_bytes())
                        .expect("Unable to write data")
                }
            }
        }
    }

    pub fn remove_file(&self) {
        match fs::remove_file(&self.todo_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while clearing todo file: {}\n", e)
            }
        }
    }

    pub fn reset(&self) {
        if !self.is_backup {
            match fs::copy(&self.todo_path, &self.todo_bk) {
                Ok(_) => self.remove_file(),
                Err(e) => {
                    eprintln!("Could't backup the todo file: {}\n", e)
                }
            }
        } else {
            self.remove_file()
        }
    }

    pub fn sort(&self) {
        let new_todo: String;
        let mut todo = String::new();
        let mut done = String::new();

        for line in self.todo_list.iter() {
            if line.len() > 5 {
                if &line[..4] == "[ ] " {
                    let line = format!("{}\n", line);
                    todo.push_str(&line);
                } else if &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    done.push_str(&line)
                }
            }
        }

        new_todo = format!("{}{}", &todo, &done);
        let mut todo_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");
        todo_file
            .write_all(new_todo.as_bytes())
            .expect("Error while trying to save the todo file")
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
                if symbol == "[*] " {
                    data = format!("{} {}\n", number, strikethrough(task));
                } else if symbol == "[ ] " {
                    data = format!("{} {}\n", number, task);
                }
                write
                    .write_all(data.as_bytes())
                    .expect("Failed to write to stdout")
            }
        }
    }
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/completed tasks in plain text, useful for scripting
        Example: todo raw done
";

pub fn help() {
    println!("{}", TODO_HELP);
}

pub fn strikethrough(text: &str) -> String {
    text.chars().flat_map(|c| vec![c, '\u{0336}']).collect()
}

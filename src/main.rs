use rc_todo::Todo;

fn main() {
    let todo = Todo::new().expect("Created todo instance failed");
    // todo.add(&["words".to_string(), "cancel subscription".to_string()]);
    todo.reset();
    let list = todo.todo_list;
    println!("{:?}", list);
    println!("Hello, world!");
}

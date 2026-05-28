use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Welcome to StudyFlow CLI");
        println!("Commands:");
        println!("add-task");
        println!("list-tasks");
        return;
    }

    match args[1].as_str() {
        "add-task" => {
            println!("Task added successfully");
        }

        "list-tasks" => {
            println!("Listing all tasks...");
        }

        _ => {
            println!("Unknown command");
        }
    }
}
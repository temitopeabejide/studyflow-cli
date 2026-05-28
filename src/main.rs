use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json");

    match data {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or(Vec::new())
        }
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();

    fs::write("tasks.json", json).expect("Unable to save tasks");
}

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
            if args.len() < 3 {
                println!("Please provide a task title");
                return;
            }

            let title = args[2].clone();

            let mut tasks = load_tasks();

            tasks.push(Task { title });

            save_tasks(&tasks);

            println!("Task added successfully");
        }

        "list-tasks" => {
            let tasks = load_tasks();

            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                for (index, task) in tasks.iter().enumerate() {
                    println!("{}. {}", index + 1, task.title);
                }
            }
        }

        _ => {
            println!("Unknown command");
        }
    }
}

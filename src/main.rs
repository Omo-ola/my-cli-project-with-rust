use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Pending,
    InProgress,
    Completed,
}
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    status: Status,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <command> [arguments]");
        println!("Commands:");
        println!("  add <description> - Add a new task");
        println!("  list - List all tasks");
        println!("  update <id> <status> - Update the status of a task");
        println!("  delete <id> - Delete a task");
        return;
    }

    let arg_1 = &args[1];

    if arg_1.to_lowercase() == "add" {
        let arg_2 = &args[2];
        if arg_2.len() == 0 {
            println!("Description cannot be empty");
            return;
        }
        let tasks = load_tasks();
        match tasks {
            Ok(mut tasks) => {
                let new_id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
                let new_task = Task {
                    id: new_id,
                    description: arg_2.to_string(),
                    status: Status::Pending,
                };
                tasks.push(new_task);
                save_tasks(&tasks).expect("Failed to save tasks");
                println!("Task added successfully");
            }
            Err(e) => {
                println!("Failed to load tasks: {}", e);
            }
        }
    }

    if arg_1.to_lowercase() == "list" {
        let tasks = load_tasks();
        match tasks {
            Ok(tasks) => {
                if tasks.is_empty() {
                    println!("No tasks found");
                } else {
                    println!("{:<5} {:<50} {:<15}", "ID", "DESCRIPTION", "STATUS");
                    println!("{}", "-".repeat(70));
                    for task in tasks {
                        println!(
                            "{:<5} {:<50} {:<15}",
                            task.id,
                            task.description,
                            format!("{:?}", task.status)
                        );
                    }
                }
            }
            Err(e) => {
                println!("Failed to load tasks: {}", e);
            }
        }
    }

    if arg_1.to_lowercase() == "complete" {
        let arg_2 = &args[2];
        if arg_2.len() == 0 {
            println!("ID cannot be empty");
            return;
        }
        let id: u32 = match arg_2.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid ID");
                return;
            }
        };
        let tasks = load_tasks();
        match tasks {
            Ok(mut tasks) => {
                let mut found = false;
                for task in tasks.iter_mut() {
                    if task.id == id {
                        task.status = Status::Completed;
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("Task with ID {} not found", id);
                    return;
                }
                save_tasks(&tasks).expect("Failed to save tasks");
                println!("Task updated successfully");
            }
            Err(e) => {
                println!("Failed to load tasks: {}", e);
            }
        }
    }

    if arg_1.to_lowercase() == "delete" {
        if args.len() < 2 {
            println!("Usage: todo delete <id>");
            return;
        }
        let arg_2 = &args[2];
        let id: u32 = match arg_2.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid ID");
                return;
            }
        };
        let tasks = load_tasks();

        match tasks {
            Ok(tasks) => {
                let filtered_tasks: Vec<Task> =
                    tasks.into_iter().filter(|task| task.id != id).collect();
                println!("Filtered tasks: {:#?}", filtered_tasks);
                save_tasks(&filtered_tasks).expect("Failed to save tasks");
                println!("Task deleted successfully");
            }
            Err(e) => {
                println!("Failed to load tasks: {}", e);
            }
        }
    }

    if arg_1.to_lowercase() == "edit" {
        if args.len() < 3 {
            println!("Provide data to update the current description");
            println!("  edit <id> <description> - edit a task");
            return;
        }
        let edit_id: u32 = match args[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid ID");
                return;
            }
        };
        let edit_desc = &args[3];

        let tasks = load_tasks();
        match tasks {
            Ok(mut tasks) => {
                println!("{:#?}", tasks);
                let mut found = false;
                for task in tasks.iter_mut() {
                    if task.id == edit_id {
                        task.description = edit_desc.to_string();
                        task.status = Status::Pending;
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("Task with ID {} not found", edit_id);
                    return;
                }
                save_tasks(&tasks).expect("Failed to save tasks");
            }
            Err(e) => {
                println!("Failed to load task ,{}", e);
                return;
            }
        }
        println!("Editted successfully");
    }
}

fn load_tasks() -> Result<Vec<Task>, io::Error> {
    if !std::path::Path::new("data.json").exists() {
        return Ok(vec![]); // no file yet — return empty vec
    }
    let data = fs::read_to_string("data.json").unwrap();
    let tasks: Vec<Task> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), io::Error> {
    let data =
        serde_json::to_string_pretty(tasks).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write("data.json", data)?;
    Ok(())
}

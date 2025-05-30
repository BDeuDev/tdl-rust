use crate::tasks::delete_task;
use colored::Colorize;
use super::Command;

const ERROR_INVALID_ID: &str = "Invalid task ID. Please provide a valid number.";

pub fn create_delete_command() -> Command {
    Command {
        name: "delete".to_string(),
        description: "Delete a task by ID".to_string(),
        usage: "tdl delete <task_id>".to_string(),
        execute: Box::new(|_args: &str| {
            match _args.trim().parse::<u64>() {
                Ok(task_id) => match delete_task(task_id) {
                    Ok(_) => println!("{}", format!("Task with ID {} deleted successfully.", task_id).green()),
                    Err(e) => println!("{}", format!("Failed to delete task with ID {}: {}", task_id, e).red()),

                }
                Err(_) => println!("{}", ERROR_INVALID_ID.red()),
            }
        }),
    }
}

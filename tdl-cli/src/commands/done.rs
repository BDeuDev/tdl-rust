use crate::tasks::complete_task;

use super::Command;

pub fn create_done_command() -> Command {
    Command {
        name: "done".to_string(),
        description: "Complete a task by ID".to_string(),
        usage: "tdl done <task_id>".to_string(),
        execute: Box::new(|_args: &str| {
            match _args.trim().parse::<u64>() {
                Ok(task_id) => match complete_task(task_id) {
                    Ok(_) => println!("Task with ID {} completed successfully!", task_id),
                    Err(e) => println!("Failed to complete task with ID {}: {}", task_id, e),
                }
                Err(_) => println!("Invalid task ID. Please provide a valid number."),
                    
            }
        }),
    }
}

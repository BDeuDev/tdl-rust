use crate::tasks::{set_priority, Priority};

use super::Command;

pub fn create_priority_command() -> Command {
    Command {
        name: "priority".to_string(),
        description: "Set the priority of a task by ID".to_string(),
        usage: "tdl priority <task_id> <priority[high, medium, low]>".to_string(),
        execute: Box::new(|args: &str| {
            let parts: Vec<&str> = args.split_whitespace().collect();
            if parts.len() != 2 {
                println!("Usage: priority <task_id> <priority>");
                return;
            }

            let task_id: u64 = match parts[0].parse() {
                Ok(id) => id,
                Err(_) => {
                    println!("Invalid task ID: {}", parts[0]);
                    return;
                }
            };

            let task_priority = match parts[1] {
                "low" => Some(Priority::Low),
                "medium" => Some(Priority::Medium),
                "high" => Some(Priority::High),
                _ => {
                    println!("Invalid priority: {}", parts[1]);
                    return;
                }
            };

            if let Err(e) = set_priority(task_id, task_priority.clone().expect("Invalid priority")) {
                println!("Error setting priority: {}", e);
            } else {
                println!("Priority set for task ID {}: {:?}", task_id, Priority::from(task_priority.clone().expect("Invalid priority")));
            }

        }), 
    }
}
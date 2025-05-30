use crate::tasks::load_tasks;

use super::Command;

pub fn create_list_command() -> Command {
    Command {
        name: "list".to_string(),
        description: "List all tasks".to_string(),
        usage: "tdl list".to_string(),
        execute: Box::new(|_args: &str| match load_tasks() {
            Ok(task) => {
                task.iter().for_each(|t| {
                    println!("Task ID: {}, Name: {}, Completed: {}, Priority: {}", t.id, t.name, t.completed, t.priority.as_ref().map_or("None".to_string(), |p| format!("{:?}", p)));
                });
            }
            Err(e) => {
                println!("Error loading tasks: {}", e);
            }
        }),
    }
}

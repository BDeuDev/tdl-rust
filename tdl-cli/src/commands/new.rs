use colored::*;

use super::Command;
use crate::tasks::add_task;

const MSG_NO_TASK_NAME: &str = "Please provide a task name.";
const MSG_TASK_CREATED: &str = "Task created successfully!";
const MSG_TASK_CREATION_FAILED: &str = "Failed to create task:";

pub fn create_new_command() -> Command {
    Command {
        name: "new".to_string(),
        description: "Create a new task".to_string(),
        usage: "tdl new <task_name>".to_string(),
        execute: Box::new(|_args| {
            if _args.is_empty() {
                println!("{}", MSG_NO_TASK_NAME.red());
            } else {
                match add_task(_args) {
                    Ok(_) => println!("{}", MSG_TASK_CREATED.green()),
                    Err(e) => println!("{} {}", MSG_TASK_CREATION_FAILED.red(), e),
                }
            }
        }),
    }
}

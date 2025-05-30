use super::Command;

pub fn create_help_command() -> Command {
    Command {
        name: "help".to_string(),
        description: "List all tasks".to_string(),
        execute: Box::new(|_args| {
            println!("Listing all tasks...{}", _args);
        }),
    }
}
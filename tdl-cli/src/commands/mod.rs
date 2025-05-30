use std::collections::HashMap;

pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub execute: Box<dyn Fn(&str) + Send + Sync>,
}

pub mod input;
pub mod list;
pub mod new;
pub mod delete;
pub mod done;
pub mod priority;

pub fn get_commands() -> HashMap<String, Command> {
    let mut commands = HashMap::new();

    commands.insert("list".to_string(), list::create_list_command());
    commands.insert("new".to_string(), new::create_new_command());
    commands.insert("delete".to_string(), delete::create_delete_command());
    commands.insert("done".to_string(), done::create_done_command());
    commands.insert("priority".to_string(), priority::create_priority_command());

    commands
}

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, ErrorKind, Read, Write, Error};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub completed: bool,
    pub priority: Option<Priority>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

const FILE_TASKS: &str =
    r"C:\Users\bdeus\OneDrive\Escritorio\Rust_vs_Go\tdl\rust-tdl\tdl-cli\src\tasks\tasks.json";

pub fn load_tasks() -> io::Result<Vec<Task>> {
    if !Path::new(FILE_TASKS).exists() {
        return Ok(vec![]);
    }

    let mut file = File::open(FILE_TASKS)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let tasks: Vec<Task> = serde_json::from_str(&content)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = File::create(FILE_TASKS)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn add_task(name: &str) -> io::Result<()> {
    let mut tasks = load_tasks()?;
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let new_task = Task {
        id: new_id,
        name: name.to_string(),
        completed: false,
        priority: None,
    };

    tasks.push(new_task);
    save_tasks(&tasks)
}

pub fn delete_task(id: u64) -> io::Result<()> {
    let mut tasks = load_tasks()?;

    let original_len = tasks.len();
    tasks.retain(|task| task.id != id);

    if tasks.len() == original_len {
        return Err(Error::new(ErrorKind::NotFound, format!("Task with ID {} not found", id)));
    }

    save_tasks(&tasks)
}

pub fn complete_task(id: u64) -> io::Result<()> {
    let mut tasks = load_tasks()?;

    for task in tasks.iter_mut() {
        if task.id == id {
            task.completed = true;
            break;
        }
    }

    save_tasks(&tasks)
}

pub fn set_priority(id: u64, priority: Priority) -> io::Result<()> {
    let mut tasks = load_tasks()?;

    for task in tasks.iter_mut() {
        if task.id == id {
            task.priority = Some(priority.clone());
            break;
        }
    }
    save_tasks(&tasks)
}
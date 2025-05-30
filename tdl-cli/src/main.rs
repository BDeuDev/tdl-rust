use std::f32::consts::E;

use colored::*;
mod commands;
mod intro;
mod outro;
mod tasks;

use commands::get_commands;
use commands::input::get_input;

const ERROR_COMMAND_NOT_FOUND: &str =
    "Command not found. Type 'tdl help' to see the available commands.";
const ERROR_INVALID_COMMAND: &str =
    "Invalid command. Type 'tdl help' to see the available commands.";

fn main() {
    intro::show_intro();

    let commands = get_commands();

    loop {
        let input = get_input("Type a command (or 'tdl exit'): ");

        if input == "tdl exit" {
            outro::show_outro();
            break;
        }
        if input == "tdl help" {
            println!("{}", "Available commands:".green());
            for (name, command) in &commands {
                println!("- tdl {}: {}", name.green(), command.description);
                println!("  Usage: {} \n", command.usage);
            }
            continue;
        }

        let mut parts = input.split_whitespace();
        let app_name = parts.next();

        if app_name != Some("tdl") {
            println!("Please, initiate the commands with 'tdl'.");
            continue;
        }

        let command = parts.next();
        let argumentos: Vec<&str> = parts.collect();

        match command {
            Some(name) => {
                if let Some(cmd) = commands.get(name) {
                    (cmd.execute)(&argumentos.join(" "));
                } else {
                    println!(
                        "{}",format!("Command '{}' not found. Type 'tdl help' to see the aviable commands.", name).red(),

                    );
                }
            }
            None => {
                println!("{}", ERROR_COMMAND_NOT_FOUND.red());
            }
        }
    }
}

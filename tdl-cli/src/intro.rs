use colored::*;

pub const WELCOME_MESSAGE: &str = "Welcome to the TDL CLI!";
pub const VERSION: &str = "1.0.0";
pub const AUTHOR: &str = "BDeuDev";
pub const LICENSE: &str = "MIT";
pub const DESCRIPTION: &str = "A simple command-line tool for managing tasks.";

pub fn show_intro() {
    println!("{}", WELCOME_MESSAGE.bold().blue());
    println!("Version: {}", VERSION.bold().yellow());
    println!("Author: {}", AUTHOR.bold().blue());
    println!("License: {}", LICENSE.bold().green());
    println!("Description: {}", DESCRIPTION.bold());
}

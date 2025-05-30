use colored::*;

pub const EXIT_MESSAGE: &str = "Thank you for using TDL CLI!";
pub const GOODBYE_MESSAGE: &str = "Goodbye!"; 

pub fn show_outro() {
    println!("{}", EXIT_MESSAGE.bold().green());
    println!("{}", GOODBYE_MESSAGE.bold().green());
}
use std::io::{self, Write};

pub fn get_input(prompt: &str) -> String {

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    input.trim().to_string()
}
use std::io::stdin;
use std::io::{self, Write};

pub fn commit_prompt() -> String {
    let mut types = String::new();
    let mut scope = String::new();
    let mut message = String::new();

    // Q1: Type of changes
    print!("Type of Change?: ");
    io::stdout().flush().unwrap();
    stdin()
        .read_line(&mut types)
        .ok()
        .expect("Failed to read line");

    // Q2: Scope of changes
    print!("Scope of changes (eg. file, function, etc)?: ");
    io::stdout().flush().unwrap();
    stdin()
        .read_line(&mut scope)
        .ok()
        .expect("Failed to read line");

    // Q3: Commit message
    print!("Commit message?: ");
    io::stdout().flush().unwrap();
    stdin()
        .read_line(&mut message)
        .ok()
        .expect("Failed to read line");

    return format!("{}({}): {}", types.trim(), scope.trim(), message.trim());
}
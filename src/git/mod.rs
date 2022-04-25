use std::io::stdin;
use std::io::{self, Write};
use std::process;

fn check_error(output: &process::Output) {
    match output.status.success() {
        true => (),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}

pub fn check_git() {
    println!("status: checking git repository...");
    let output = process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .expect("error: failed to check git repository in this directory");

    check_error(&output);
}

pub fn add_all_stage() {
    println!("status: staging all changes in this directory...");
    let output = process::Command::new("git")
        .args(["add", "."])
        .output()
        .expect("error: failed to stage all changes in this directory");

    check_error(&output);
}

pub fn input_commit_messages() -> String {
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

pub fn commit(message: &String) {
    println!("status: comitting changes...");
    let output = process::Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .expect("error: failed to commit changes");

    check_error(&output);
}

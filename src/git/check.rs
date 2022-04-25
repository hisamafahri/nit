use crate::git::error;
use std::process;

pub fn check_git() {
    println!("status: checking git repository...");
    let output = process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .expect("error: failed to check git repository in this directory");
    error::handle(&output);
}

pub fn check_branch() -> String {
    println!("status: checking branches...");
    let output = process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("error: failed to check git repository in this directory");

    match output.status.success() {
        true => {
            format!("{}", String::from_utf8_lossy(&output.stdout).trim())
        },
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}

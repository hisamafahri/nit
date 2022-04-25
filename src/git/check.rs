use std::process;
use crate::git::error;

pub fn check_git() {
    println!("status: checking git repository...");
    let output = process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .expect("error: failed to check git repository in this directory");
        
    error::handle(&output);
}
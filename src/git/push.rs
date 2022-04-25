use std::process;
use crate::git::error;

pub fn push(branch: &String) {
    println!("status: pushing changes...");
    let output = process::Command::new("git")
        .args(["push", "origin", &branch])
        .output()
        .expect("error: failed to push changes");

    error::handle(&output);
}
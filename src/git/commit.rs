use std::process;
use crate::git::error;

pub fn commit(message: &String) {
    println!("status: comitting changes...");
    let output = process::Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .expect("error: failed to commit changes");

    error::handle(&output);
}
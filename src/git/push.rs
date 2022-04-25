use std::process;
use crate::git::error;

pub fn push() {
    println!("status: pushing changes...");
    let output = process::Command::new("git")
        .args(["push", "origin", "main"])
        .output()
        .expect("error: failed to push changes");

    error::handle(&output);
}
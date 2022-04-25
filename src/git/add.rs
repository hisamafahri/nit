use std::process;
use crate::git::error;

pub fn stage_all() {
    println!("status: staging all changes in this directory...");
    let output = process::Command::new("git")
        .args(["add", "."])
        .output()
        .expect("error: failed to stage all changes in this directory");

    error::handle(&output);
}
use crate::git::error;
use crate::helper;
use std::process;

pub fn check_git() {
    println!("status: checking git repository...");
    let args = [
        String::from("rev-parse"),
        String::from("--is-inside-work-tree"),
    ];
    let output = helper::run(&String::from("git"), &args);

    error::handle(&output);
}

pub fn check_branch() -> String {
    println!("status: checking branches...");
    let args = [
        String::from("rev-parse"),
        String::from("--abbrev-ref"),
        String::from("HEAD"),
    ];
    let output = helper::run(&String::from("git"), &args);

    match output.status.success() {
        true => {
            format!("{}", String::from_utf8_lossy(&output.stdout).trim())
        }
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}

fn check_aliases() -> String {
    println!("status: checking aliases...");
    let args = [String::from("remote")];
    let output = helper::run(&String::from("git"), &args);

    match output.status.success() {
        true => {
            format!("{}", String::from_utf8_lossy(&output.stdout).trim())
        }
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}

pub fn check_remote() {
    let aliases = check_aliases();
    let aliases_split: Vec<&str> = aliases.split_whitespace().collect();
    for alias in aliases_split {
        let args = [
            String::from("remote"),
            String::from("get-url"),
            String::from("--push"),
            String::from(alias.trim()),
        ];
        let output = helper::run(&String::from("git"), &args);
        match output.status.success() {
            true => {
                println!(
                    "{}: {}",
                    alias,
                    String::from_utf8_lossy(&output.stdout).trim()
                )
            }
            false => {
                println!("{}", String::from_utf8_lossy(&output.stderr).trim());
                process::exit(1)
            }
        }
    }
}

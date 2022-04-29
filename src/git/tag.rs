use crate::helper;
use std::process;
use futures_lite::future;

pub fn add(tag: &String) {
    println!("\x1B[38;5;245m info \x1B[0m creating a new tag...");
    let args = [
        String::from("tag"),
        String::from(tag),
    ];
    let output = helper::command::run(&String::from("git"), &args);
    helper::output::print_err(&output);
}

pub fn list() -> std::vec::Vec<std::string::String> {
    println!("\x1B[38;5;245m info \x1B[0m checking tags...");
    let mut tags = vec![];

    let args = [
        String::from("tag"),
    ];
    let output = helper::command::run(&String::from("git"), &args);
    let tags_string = helper::output::handle(&output);

    let tags_split: Vec<&str> = tags_string.split_whitespace().collect();
    for tag in tags_split {
        tags.push(format!("{}", tag.trim()));
    }
    tags
}

pub fn delete_local(tag: &String) {
    println!("\x1B[38;5;245m info \x1B[0m deleting local tag \"{}\"...", tag);
    let args = [
        String::from("tag"),
        String::from("--delete"),
        String::from(tag),
    ];
    let output = helper::command::run(&String::from("git"), &args);
    helper::output::print_err(&output);
}

pub fn delete_remote(remote: &String, tag: &String) {
    println!("\x1B[38;5;245m info \x1B[0m pushing changes...");
    future::block_on(async {
        let cmd = process::Command::new("git")
            .args(["push", "--delete", &remote.trim(), &tag.trim()])
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .output()
            .expect("\x1B[38;5;1m error \x1B[0m failed to run command");

        println!("{}", String::from_utf8_lossy(&cmd.stderr).trim());
        println!("{}", String::from_utf8_lossy(&cmd.stdout).trim());
    });
}
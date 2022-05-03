use crate::git;
use crate::helper;

pub fn pull() {
    git::check::directory();
    let remote = git::check::remote();
    let branch = git::check::branch();
    let selected_remote = helper::prompt::build::select(
        &String::from("Where you want to get the updates from?"),
        &remote,
    );
    let selected_remote_split: Vec<&str> = selected_remote.split(": ").collect();
    git::pull::pull(&String::from(selected_remote_split[0]), &branch);
    println!("\x1B[38;5;2m success \x1B[0m updates pulled successfully!");
}
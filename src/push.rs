use crate::git;
use crate::helper;

pub fn push() {
    git::check::directory();
    let branch = git::check::branch();
    let remote = git::check::remote();
    let selected_remote = helper::prompt::build::select(
        &String::from("Where you want to push your changes?"),
        &remote,
    );
    let selected_remote_split: Vec<&str> = selected_remote.split(": ").collect();
    git::push::push(&String::from(selected_remote_split[0]), &branch);
    println!("\x1B[38;5;2m success \x1B[0m changes successfully pushed!");
}
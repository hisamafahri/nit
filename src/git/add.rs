use crate::helper;

pub fn stage_all() {
    println!("\x1B[7m STATUS: \x1B[0m staging all changes in this directory...");
    let args = [String::from("add"), String::from(".")];
    let output = helper::command::run(&String::from("git"), &args);

    helper::output::ignore(&output);
}
use crate::helper;

pub fn stage_all() {
    println!("status: staging all changes in this directory...");
    let args = [String::from("add"), String::from(".")];
    let output = helper::command_run(&String::from("git"), &args);

    helper::output_print(&output);
}
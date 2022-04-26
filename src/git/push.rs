use crate::helper;

pub fn push(remote: &String, branch: &String) {
    println!("\x1B[7m STATUS: \x1B[0m pushing changes...");

    let args = [
        String::from("push"),
        String::from(remote),
        String::from(branch),
    ];
    let output = helper::command::run(&String::from("git"), &args);

    helper::output::print(&output);
}

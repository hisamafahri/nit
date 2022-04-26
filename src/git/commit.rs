use crate::helper;

pub fn commit(message: &String) {
    println!("\x1B[7m STATUS: \x1B[0m committing changes...");
    let args = [
        String::from("commit"),
        String::from("-m"),
        String::from(message),
    ];
    let output = helper::command::run(&String::from("git"), &args);

    helper::output::print(&output);
}

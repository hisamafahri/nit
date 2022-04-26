use crate::helper;
use futures_lite::future;

pub fn push(remote: &String, branch: &String) {
    println!("\x1B[7m STATUS: \x1B[0m pushing changes...");

    let args = [
        String::from("push"),
        String::from(remote),
        String::from(branch),
    ];
    future::block_on(async {
        let output = helper::command::run(&String::from("git"), &args);
        helper::output::print(&output);
    });
}

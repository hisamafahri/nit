use crate::git::error;
use crate::helper;

pub fn commit(message: &String) {
    println!("status: comitting changes...");
    let args = [
        String::from("commit"),
        String::from("-m"),
        String::from(message),
    ];
    let output = helper::run(&String::from("git"), &args);

    error::handle(&output);
}

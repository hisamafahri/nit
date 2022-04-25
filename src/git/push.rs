use crate::helper;

pub fn push(branch: &String) {
    println!("status: pushing changes...");

    let args = [
        String::from("push"),
        String::from("origin"),
        String::from(branch),
    ];
    let output = helper::run(&String::from("git"), &args);

    helper::handler(&output, ());
}

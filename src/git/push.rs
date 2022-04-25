use crate::helper;

pub fn push(remote: &String, branch: &String) {
    println!("status: pushing changes...");

    let args = [
        String::from("push"),
        String::from(remote),
        String::from(branch),
    ];
    let output = helper::command_run(&String::from("git"), &args);

    helper::output_print(&output);
}

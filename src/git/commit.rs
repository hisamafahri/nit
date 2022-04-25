use crate::helper;

pub fn commit(message: &String) {
    println!("status: comitting changes...");
    let args = [
        String::from("commit"),
        String::from("-m"),
        String::from(message),
    ];
    let output = helper::command_run(&String::from("git"), &args);

    helper::output_print(&output);
}

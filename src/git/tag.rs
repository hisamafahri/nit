use crate::helper;

pub fn add(tag: &String) {
    println!("\x1B[38;5;245m info \x1B[0m creating a new tag...");
    let args = [
        String::from("tag"),
        String::from(tag),
    ];
    let output = helper::command::run(&String::from("git"), &args);
    helper::output::print_err(&output);
}

pub fn list() -> std::vec::Vec<std::string::String> {
    println!("\x1B[38;5;245m info \x1B[0m checking tags...");
    let mut tags = vec![];

    let args = [
        String::from("tag"),
    ];
    let output = helper::command::run(&String::from("git"), &args);
    let tags_string = helper::output::handle(&output);

    let tags_split: Vec<&str> = tags_string.split_whitespace().collect();
    for tag in tags_split {
        tags.push(format!("{}", tag));
    }
    tags
}
use crate::helper;

pub fn directory() {
    println!("\x1B[38;5;245m info \x1B[0m checking current directory...");
    let args = [
        String::from("rev-parse"),
        String::from("--is-inside-work-tree"),
    ];
    let output = helper::command::run(&String::from("git"), &args);

    helper::output::ignore(&output);
    println!("\x1B[38;5;2m success \x1B[0m directory is inside a git work tree!");
}

pub fn branch() -> String {
    println!("\x1B[38;5;245m info \x1B[0m checking branches...");
    let args = [
        String::from("rev-parse"),
        String::from("--abbrev-ref"),
        String::from("HEAD"),
    ];
    let output = helper::command::run(&String::from("git"), &args);

    return helper::output::handle(&output);
}

fn aliases() -> String {
    println!("\x1B[38;5;245m info \x1B[0m checking aliases...");
    let args = [String::from("remote")];
    let output = helper::command::run(&String::from("git"), &args);

    return helper::output::handle(&output);
}

pub fn remote() -> std::vec::Vec<std::string::String> {
    let mut places = vec![];
    let aliases = aliases();
    let aliases_split: Vec<&str> = aliases.split_whitespace().collect();
    for alias in aliases_split {
        let args = [
            String::from("remote"),
            String::from("get-url"),
            String::from("--push"),
            String::from(alias.trim()),
        ];
        let output = helper::command::run(&String::from("git"), &args);

        let result = helper::output::handle(&output);
        places.push(format!("{}: {}", alias, result));
    };
    places
}

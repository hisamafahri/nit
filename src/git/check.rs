use crate::helper;

pub fn directory() {
    println!("status: checking git repository...");
    let args = [
        String::from("rev-parse"),
        String::from("--is-inside-work-tree"),
    ];
    let output = helper::command::run(&String::from("git"), &args);

    helper::output::ignore(&output);
}

pub fn branch() -> String {
    println!("status: checking branches...");
    let args = [
        String::from("rev-parse"),
        String::from("--abbrev-ref"),
        String::from("HEAD"),
    ];
    let output = helper::command::run(&String::from("git"), &args);

    return helper::output::handle(&output);
}

fn aliases() -> String {
    println!("status: checking aliases...");
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

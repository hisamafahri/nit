use crate::helper;

pub fn check_git() {
    println!("status: checking git repository...");
    let args = [
        String::from("rev-parse"),
        String::from("--is-inside-work-tree"),
    ];
    let output = helper::run(&String::from("git"), &args);

    helper::handler(&output, ());
}

pub fn check_branch() -> String {
    println!("status: checking branches...");
    let args = [
        String::from("rev-parse"),
        String::from("--abbrev-ref"),
        String::from("HEAD"),
    ];
    let output = helper::run(&String::from("git"), &args);

    return helper::handler_string(&output);
}

fn check_aliases() -> String {
    println!("status: checking aliases...");
    let args = [String::from("remote")];
    let output = helper::run(&String::from("git"), &args);

    return helper::handler_string(&output);
}

pub fn check_remote() {
    let aliases = check_aliases();
    let aliases_split: Vec<&str> = aliases.split_whitespace().collect();
    for alias in aliases_split {
        let args = [
            String::from("remote"),
            String::from("get-url"),
            String::from("--push"),
            String::from(alias.trim()),
        ];
        let output = helper::run(&String::from("git"), &args);

        helper::handler_string(&output);
    }
}

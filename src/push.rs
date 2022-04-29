use crate::git;
use crate::helper;
use std::process;


pub fn push() {
    git::check::directory();
    let remote = git::check::remote();
    let push_options = vec![
        String::from("changes: Push your changes to the remote repository"),
        String::from("tag: Push a tag to the remote repository"),
    ];
    let selected_push_options =
        helper::prompt::build::select(&String::from("What do you want to push?"), &push_options);

    // Push changes
    if selected_push_options == push_options[0] {
        let branch = git::check::branch();
        let selected_remote = helper::prompt::build::select(
            &String::from("Where you want to push your changes?"),
            &remote,
        );
        let selected_remote_split: Vec<&str> = selected_remote.split(": ").collect();
        git::push::push(&String::from(selected_remote_split[0]), &branch);
        println!("\x1B[38;5;2m success \x1B[0m changes successfully pushed!");
    }

    // Push tag
    if selected_push_options == push_options[1] {
        let tags = git::tag::list();

        // if no tags are detected
        if tags.is_empty() {
            println!("\x1B[38;5;1m error \x1B[0m no tags found!");
            process::exit(1)
        }
        
        let selected_tag = helper::prompt::build::select(
            &String::from("Which tag you want to push?"),
            &tags,
        );

        let selected_remote = helper::prompt::build::select(
            &String::from("Where you want to push your changes?"),
            &remote,
        );

        let selected_remote_split: Vec<&str> = selected_remote.split(": ").collect();
        git::push::push(&String::from(selected_remote_split[0]), &selected_tag);
        println!("\x1B[38;5;2m success \x1B[0m changes successfully pushed!");
    }


}

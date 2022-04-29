use crate::git;
use crate::helper;
use std::process;


pub fn tag() {
    git::check::directory();
    let tag_options = vec![
        String::from("list: List all tags"),
        String::from("add: Add a new tag"),
        String::from("push: Push a tag to remote repository"),
        String::from("delete: Remove an existing tag"),
    ];
    let selected_tag_options = helper::prompt::build::select(
        &String::from("What do you want to do with tags?"),
        &tag_options,
    );

    // If user want to list all tags
    if selected_tag_options == tag_options[0] {
        let tags = git::tag::list();

        // if no tags are detected
        if tags.is_empty() {
            println!("\x1B[38;5;1m error \x1B[0m no tags found!");
            process::exit(1)
        }

        println!(""); //  add spacing
        for tag in tags {
            println!("{}", tag);
        }
        println!(""); //  add spacing
        println!("\x1B[38;5;2m success \x1B[0m tags listed successfully!");
    }

    // If user want to add a new tag
    if selected_tag_options == tag_options[1] {
        let new_tag = helper::prompt::build::input(
            &String::from("Tag name?"),
        );
        git::tag::add(&new_tag);
        println!("\x1B[38;5;2m success \x1B[0m tag created successfully!");
    }

    // If user want to push a tag
    if selected_tag_options == tag_options[2] {
        println!("\x1B[38;5;2m success \x1B[0m push tag command is called!");
    }

    // If user want to delete a tag
    if selected_tag_options == tag_options[3] {
        let tags = git::tag::list();

        // if no tags are detected
        if tags.is_empty() {
            println!("\x1B[38;5;1m error \x1B[0m no tags found!");
            process::exit(1)
        }

        let selected_tag = helper::prompt::build::select(
            &String::from("Which tag you want to delete?"),
            &tags,
        );
        git::tag::delete(&selected_tag);
        println!("\x1B[38;5;2m success \x1B[0m tag \"{}\" deleted successfully!", selected_tag);
    }
}
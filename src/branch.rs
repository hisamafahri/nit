use crate::git;
use crate::helper;
use std::process;

pub fn branch() {
    git::check::directory();
    let branch_options = vec![
        String::from("list: List all branches"),
        String::from("checkout: Checkout to another branch"),
        String::from("add: Add a new branch"),
        String::from("delete: Remove an existing branch"),
    ];
    let selected_branch_options = helper::prompt::build::select(
        &String::from("What do you want to do with branches?"),
        &branch_options,
    );

    // If user want to list all branches
    if selected_branch_options == branch_options[0] {
        println!("User want to list the existing branches");
        process::exit(1);
    }

    // If user want to switch branches
    if selected_branch_options == branch_options[1] {
        println!("User want to switch branches");
        process::exit(1);
    }
    
    if selected_branch_options == branch_options[2] {
        println!("User want to create a new branch"):
        process::exit(1);
    }
    
    if selected_branch_options == branch_options[3] {
        println!("User want to delete a branch"):
        process::exit(1);
    }
}

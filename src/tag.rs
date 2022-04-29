use crate::git;
use crate::helper;

pub fn tag() {
    git::check::directory();
    let tag_options = vec![
        String::from("add: Add a new tag"),
        String::from("push: Push a tag to remote repository"),
        String::from("delete: Remove an existing tag"),
    ];
    let selected_remote = helper::prompt::build::select(
        &String::from("What do you want to do with tags?"),
        &tag_options,
    );
    println!("{}", selected_remote);
}
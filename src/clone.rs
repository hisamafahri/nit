use crate::git;
use crate::helper;
use std::process;
use url::Url;

pub fn clone() {
    let repo_url = helper::prompt::build::input(&String::from("Repository URL?"));

    let parsed = Url::parse(&repo_url);

    // If inputted url is not a valid url
    if parsed.is_err() {
        println!("\x1B[38;5;1m error \x1B[0m value is not a valid url!");
        process::exit(1)
    }

    git::clone::clone(&repo_url);
}

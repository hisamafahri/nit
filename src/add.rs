use crate::cli;
use crate::git;
use crate::helper;
use std::process;

pub fn add(commit: &cli::Add) {
    git::check::directory();
    if commit.all {
        git::add::stage_all();
        println!("\x1B[38;5;2m success \x1B[0m all changes staged successfully!");
        process::exit(1)
    }
    let files = git::check::changes();
    
    if files.is_empty() {
        println!("\x1B[38;5;1m error \x1B[0m no changes detected");
        process::exit(1)
    }

    let index_selected = helper::prompt::build::multi_select(
        &String::from("Which files you want to staged?"),
        &files,
    );

    if index_selected.is_empty() {
        println!("\x1B[38;5;1m error \x1B[0m no file selected");
        process::exit(1)
    }

    println!("\x1B[38;5;245m info \x1B[0m staging selected changes...");
    for index in index_selected {
        git::add::stage(&String::from(files[index].trim()));
    }
    println!("\x1B[38;5;2m success \x1B[0m selected changes staged successfully!");


}
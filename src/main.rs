use clap::Parser;
use std::process;
mod cli;
mod git;
mod helper;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit(all),
        cli::Commands::Push => push(),
        cli::Commands::Add(all) => add(all),
        cli::Commands::Tag => tag(),
    }
}

fn commit(commit: &cli::Commit) {
    git::check::directory();
    let commit_message = helper::prompt::commit::commit_prompt();
    if commit.all {
        git::add::stage_all();
        println!("\x1B[38;5;2m success \x1B[0m changes staged successfully!");
    }
    git::commit::commit(&commit_message);
    println!("\x1B[38;5;2m success \x1B[0m changes successfully committed!");
}

fn push() {
    git::check::directory();
    let branch = git::check::branch();
    let remote = git::check::remote();
    let selected_remote = helper::prompt::build::select(
        &String::from("Where you want to push your changes?"),
        &remote,
    );
    let selected_remote_split: Vec<&str> = selected_remote.split(": ").collect();
    git::push::push(&String::from(selected_remote_split[0]), &branch);
    println!("\x1B[38;5;2m success \x1B[0m changes successfully pushed!");
}

fn add(commit: &cli::Add) {
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

fn tag() {
    println!("Tag command is called!")
}
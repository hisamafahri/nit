use clap::Parser;
use std::process;
mod cli;
mod git;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit_all(all),
    }
}

fn commit_all(commit: &cli::Commit) {
    println!("all: {:?}", commit.all);
    let is_repo = git::check_git();
    if !is_repo {
        process::exit(1);
    }
    println!("if error, should not be reachable")
}

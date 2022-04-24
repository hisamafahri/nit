use clap::{Parser};
mod cli;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit_all(all)
    }

}

fn commit_all(commit: &cli::Commit) {
    println!("all: {:?}", commit.all);
    println!("commit all files");
}
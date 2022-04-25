use clap::Parser;
mod cli;
mod git;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit(all),
    }
}

fn commit(commit: &cli::Commit) {
    git::check_git();
    if commit.all {
        git::add_all_stage()
    }
    let commit_message = git::input_commit_messages();
    println!("{}", commit_message);
    git::commit(&commit_message);
}

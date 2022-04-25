use clap::Parser;
mod cli;
mod git;
mod helper;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit(all),
        cli::Commands::Push => push()
    }
}

fn commit(commit: &cli::Commit) {
    git::check_git();
    if commit.all {
        git::stage_all()
    }
    let commit_message = helper::commit_prompt();
    git::commit(&commit_message);
}

fn push() {
    git::check_git();
    let branch = git::check_branch();
    println!("branch: {}", branch);
    git::check_remote();
    git::push(&branch);
}
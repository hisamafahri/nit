use clap::Parser;
mod cli;
mod git;
mod helper;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit(all),
        cli::Commands::Push => push(),
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
    let remote = git::check_remote();
    let selected_remote = helper::select(
        &String::from("Where you want to push your changes?"),
        &remote,
    );
    let selected_remote_split: Vec<&str> = selected_remote.split(": ").collect();
    git::push(&String::from(selected_remote_split[0]), &branch);
}

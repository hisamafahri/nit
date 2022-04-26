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
    git::check::directory();
    let commit_message = helper::prompt::commit::commit_prompt();
    if commit.all {
        git::add::stage_all()
    }
    git::commit::commit(&commit_message);
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
}

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

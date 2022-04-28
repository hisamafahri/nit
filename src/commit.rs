use crate::cli;
use crate::git;
use crate::helper;

pub fn commit(commit: &cli::Commit) {
    git::check::directory();
    let commit_message = helper::prompt::commit::commit_prompt();
    if commit.all {
        git::add::stage_all();
        println!("\x1B[38;5;2m success \x1B[0m changes staged successfully!");
    }
    git::commit::commit(&commit_message);
    println!("\x1B[38;5;2m success \x1B[0m changes successfully committed!");
}
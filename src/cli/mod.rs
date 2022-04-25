use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "nit")]
#[clap(author = "Hisam Fahri <me@hisamafahri.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "nit helps you wrap your git commands")]
#[clap(long_about = "Wrap your git commands with nit to make your workflow faster and more consistent")]
#[clap(propagate_version = true)]

pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Commit staged changes on the current working directory
    Commit(Commit),

    /// Push changes into remote repository
    Push
}

#[derive(Parser)]
pub struct Commit {
    /// Commit all changes (staged & unstaged changes) on the current working directory
    #[clap(short, long)]
    pub all: bool, 
}
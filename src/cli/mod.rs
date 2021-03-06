use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "nit")]
#[clap(author = "Hisam Fahri <me@hisamafahri.com>")]
#[clap(version = "0.1.4")]
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
    #[clap(alias = "c")]
    Commit(Commit),
    
    /// Push changes or tag into remote repository
    #[clap(alias = "p")]
    Push,

    /// Staged changes
    #[clap(alias = "a")]
    Add(Add),

    /// Work with tag object
    #[clap(alias = "t")]
    Tag,

    /// Clone a remote repository
    #[clap(alias = "cl")]
    Clone,

    /// Pull updates from remote repository
    #[clap(alias = "pl")]
    Pull,
}

#[derive(Parser)]
pub struct Commit {
    /// Commit all changes (staged & unstaged changes) on the current working directory
    #[clap(short, long)]
    pub all: bool, 
}

#[derive(Parser)]
pub struct Add {
    /// Staged all changes
    #[clap(short, long)]
    pub all: bool, 
}
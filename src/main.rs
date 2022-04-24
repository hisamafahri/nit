use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(name = "nit")]
#[clap(author = "Hisam Fahri <me@hisamafahri.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "nit helps you wrap your git commands")]
#[clap(long_about = "Wrap your git commands with nit to make your workflow faster and more consistent")]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Commit staged changes on the current working directory
    Commit(Commit),
}

#[derive(Parser)]
struct Commit {
    /// Commit all changes (staged & unstaged changes) on the current working directory
    #[clap(short, long)]
    all: bool, 
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Commit(all) => commit_all(all)
    }

}

fn commit_all(commit: &Commit) {
    println!("all: {:?}", commit.all);
    println!("commit all files");
}
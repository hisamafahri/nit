use clap::Parser;
mod cli;
mod git;
mod helper;

mod commit;
mod push;
mod add;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit::commit(all),
        cli::Commands::Push => push::push(),
        cli::Commands::Add(all) => add::add(all),
        cli::Commands::Tag => tag(),
    }
}

fn tag() {
    println!("Tag command is called!")
}
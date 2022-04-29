use clap::Parser;
mod cli;
mod git;
mod helper;

mod commit;
mod push;
mod add;
mod tag;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit::commit(all),
        cli::Commands::Push => push::push(),
        cli::Commands::Add(all) => add::add(all),
        cli::Commands::Tag => tag::tag(),
    }
}
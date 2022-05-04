use clap::Parser;
mod cli;
mod git;
mod helper;

mod commit;
mod push;
mod add;
mod tag;
mod clone;
mod pull;
mod branch;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Commit(all) => commit::commit(all),
        cli::Commands::Push => push::push(),
        cli::Commands::Add(all) => add::add(all),
        cli::Commands::Tag => tag::tag(),
        cli::Commands::Clone => clone::clone(),
        cli::Commands::Pull => pull::pull(),
        cli::Commands::Branch => branch::branch(),
    }
}
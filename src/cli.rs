use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
pub struct New {
    name: String,
    version: String,
}

#[derive(Args, Debug)]
pub struct Run {
    name: String,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    New(New),
    Run(Run),
    List,
}

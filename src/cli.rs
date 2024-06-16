use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct New {
    pub name: String,
    pub version: String,
}

#[derive(Args, Debug)]
pub struct Run {
    pub name: String,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    New(New),
    Run(Run),
    List,
}

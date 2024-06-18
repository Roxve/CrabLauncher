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

#[derive(Debug, Subcommand)]
pub enum Commands {
    New(New),
    Run { name: String },
    Del { name: String },
    List,
}

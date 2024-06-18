use std::fs::{self, read};

use clap::Parser;

use client::download::download;
use profiles::{read_profile_setup, Profile};

mod cli;
mod config;
mod env;

mod client;
mod error;
mod json;

mod profiles;

use cli::Cli;
use env::Env;

pub const LAUNCHER_DIR: &str = "launcher/";
pub const LIB_DIR: &str = "launcher/libraries/";
pub const ASSETS_DIR: &str = "launcher/assets/";
pub const PROFILES_DIR: &str = "launcher/profiles/";

pub const OS: &str = if cfg!(target_os = "windows") {
    "windows"
} else if cfg!(target_os = "macos") {
    "osx"
} else if cfg!(target_os = "linux") {
    "linux"
} else {
    "unknown"
};

pub const ARCH: &str = if cfg!(target_arch = "x86") {
    "x86"
} else if cfg!(target_arch = "x86_64") {
    "x86_64"
} else if cfg!(target_arch = "arm") {
    "arm"
} else if cfg!(target_arch = "aarch64") {
    "aarch_64"
} else {
    "unknown"
};

use crate::json::manifest::Manifest;

fn init() {
    fs::create_dir_all(LIB_DIR).expect("failed to create libraries folder");
    fs::create_dir_all(ASSETS_DIR).expect("failed to create assest folder");
}

fn main() {
    init();

    let manifest = Manifest::init_manifest();
    let parse = Cli::try_parse().unwrap_or(Cli {
        command: cli::Commands::List,
    });
    let mut env = Env::from_manifest(manifest);

    match parse.command {
        cli::Commands::New(new) => env.add_profile(Profile {
            name: new.name,
            version: new.version,
        }),

        cli::Commands::Run { name } => {
            println!("downloading....");
            download(read_profile_setup(name));
            println!("downloading: OK\nrunning....");
        }

        cli::Commands::Del { name } => env.del_profile(name),

        cli::Commands::List => {
            println!("__PROFILES__");
            for profile in env.profiles {
                println!("{}:\tversion: {}", profile.name, profile.version);
            }
        }
    }
}

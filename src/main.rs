use std::fs;

use clap::Parser;

use client::download::download;
use java::list;
use json::client::{Arch, OsName};
use profiles::{read_profile_setup, Profile};

mod cli;
mod config;
mod env;

mod client;
mod error;
mod json;

mod java;
mod profiles;

use cli::Cli;
use env::Env;

pub const LAUNCHER_DIR: &str = "launcher/";
pub const LIB_DIR: &str = "launcher/libraries/";
pub const ASSETS_DIR: &str = "launcher/assets/";
pub const PROFILES_DIR: &str = "launcher/profiles/";

pub const OS: OsName = if cfg!(target_os = "windows") {
    OsName::Windows
} else if cfg!(target_os = "macos") {
    OsName::Osx
} else if cfg!(target_os = "linux") {
    OsName::Linux
} else {
    panic!("unsupported OS!")
};

pub const ARCH: Arch = if cfg!(target_arch = "x86") {
    Arch::X86
} else if cfg!(target_arch = "x86_64") {
    Arch::X86_64
} else if cfg!(target_arch = "aarch64") {
    Arch::ARM64
} else {
    panic!("unsupported arch")
};

use crate::json::manifest::Manifest;

fn init() {
    fs::create_dir_all(LIB_DIR).expect("failed to create libraries folder");
    fs::create_dir_all(ASSETS_DIR).expect("failed to create assest folder");
}

fn main() {
    init();
    list();
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
            download(read_profile_setup(name.clone()));
            println!("downloading: OK\nrunning....");
            env.run(name);
            println!("FAILED or closed...");
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

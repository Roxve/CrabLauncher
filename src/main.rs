use std::{
    fs::{self, File},
    io::{Read, Write},
};

use clap::Parser;

use download::download;
use profiles::{read_profile_setup, Profile};
use reqwest;

mod cli;
mod download;
mod env;
mod error;
mod manifest;
mod profiles;
mod rule;
mod setup;

use cli::Cli;
use env::Env;
pub const LAUNCHER_DIR: &str = "launcher";
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

fn init() {
    fs::create_dir_all(LIB_DIR).expect("failed to create libraries folder");
    fs::create_dir_all(ASSETS_DIR).expect("failed to create assest folder");
}

fn init_manifest() -> manifest::Manifest {
    // download version info
    let res =
        reqwest::blocking::get("https://launchermeta.mojang.com/mc/game/version_manifest.json");
    // if offline use pre-downloaded file
    let manifest = if res.is_ok() {
        let res = res.unwrap();

        let buffer = res.text().unwrap();
        let bytes = buffer.as_bytes();

        let mut manifest_file = File::create("launcher/version_manifest.json").unwrap();

        manifest_file
            .write_all(bytes)
            .expect("failed writing file version_manifest.json");

        serde_json::from_str(buffer.as_str()).expect("failed reading file version_manifest.json")
    } else {
        let mut manifest_file = File::open("launcher/version_manifest.json").expect(
            "failed to load version_manifest.json, please connect to the internet and try again",
        );

        let mut buffer = String::new();
        manifest_file
            .read_to_string(&mut buffer)
            .expect("opened version_manifest.json, but failed reading it");

        serde_json::from_str(buffer.as_str()).expect("failed parsing file version_manifest.json")
    };

    manifest
}

fn main() {
    init();
    let manifest = init_manifest();
    let parse = Cli::try_parse().unwrap_or(Cli {
        command: cli::Commands::List,
    });
    let mut env = Env::from_manifest(manifest);

    match parse.command {
        cli::Commands::New(new) => env.add_profile(Profile {
            name: new.name,
            version: new.version,
        }),
        cli::Commands::Run(_) => todo!(),
        cli::Commands::List => {
            println!("__PROFILES__");
            for profile in env.profiles {
                println!("{}:\tversion: {}", profile.name, profile.version);
            }
        }
    }

    let setup = read_profile_setup("test".to_string());
    download(setup);
    // let prof = fs::read_to_string("launcher/profiles/test/test.json").unwrap();

    // let o: Setup = serde_json::from_str(prof.as_str()).unwrap();
    // dbg!(&o);
}

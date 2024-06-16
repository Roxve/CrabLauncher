use std::{
    fs::{self, File},
    io::{Read, Write},
};

use clap::{builder::Str, Parser};
use launch::Arguments;
use profiles::Profile;
use reqwest;

mod cli;
mod env;
mod launch;
mod manifest;
mod profiles;

use cli::Cli;
use env::Env;
use serde_json::{Map, Value};

fn init() {
    fs::create_dir_all("launcher/libraries").expect("failed to create libraries folder");
    fs::create_dir_all("launcher/assests").expect("failed to create assest folder");
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
    Cli::try_parse();
    let mut env = Env::from_manifest(manifest);

    env.add_profile(Profile {
        name: "test".to_owned(),
        version: "1.21".to_owned(),
    });

    let prof = fs::read_to_string("launcher/profiles/test/test.json").unwrap();

    let obj: Map<String, Value> = serde_json::from_str(prof.as_str()).unwrap();

    let o: Arguments = serde_json::from_str(obj["arguments"].clone().to_string().as_str()).unwrap();
    dbg!(&o);
}

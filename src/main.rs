use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
};

use reqwest;
use serde_json::{Map, Value};

mod env;
mod instances;
use env::Env;

fn init() {
    fs::create_dir_all("launcher/ibs").expect("failed to create libs folder");
    fs::create_dir_all("launcher/assests").expect("failed to create assest folder");
}

fn init_manifest() -> Map<String, Value> {
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

    let env = Env::from_manifest(manifest);
}

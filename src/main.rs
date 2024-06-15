use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use reqwest;
use serde_json::Value;

#[derive(Debug)]
pub enum VersionKind {
    Release,
    Snapshot,
    OldAlpha,
    OldBeta,
}
#[derive(Debug)]
pub struct Version {
    pub kind: VersionKind,
    pub url: String,
}
#[derive(Debug)]
pub struct Env {
    pub versions: HashMap<String, Version>,
    pub latest: String,
}

impl Env {
    pub fn new(versions: HashMap<String, Version>, latest: String) -> Self {
        Self { versions, latest }
    }
}

fn main() {
    fs::create_dir_all("launcher/ibs").expect("failed to create libs folder");
    fs::create_dir_all("launcher/assests").expect("failed to create assest folder");

    // download version info
    let res =
        reqwest::blocking::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
            .expect("failed downloading minecraft version manifest");

    let buffer = res.text().unwrap();
    let bytes = buffer.as_bytes();

    let mut manifest = File::create("launcher/version_manifest.json").unwrap();

    manifest
        .write_all(bytes.clone())
        .expect("failed writing file version_manifest.json");

    let manifest: serde_json::Value =
        serde_json::from_str(buffer.as_str()).expect("failed reading file version_manifest.json");

    let manifest = manifest.as_object().unwrap();
    let mut versions: HashMap<String, Version> = HashMap::new();

    for obj in manifest["versions"].as_array().unwrap() {
        let obj = obj.as_object().unwrap();
        let id = obj["id"].as_str().unwrap().to_string();

        let kind = match &obj["type"] {
            Value::String(s) => match s.as_str() {
                "release" => VersionKind::Release,
                "snapshot" => VersionKind::Snapshot,
                "old_alpha" => VersionKind::OldAlpha,
                "old_beta" => VersionKind::OldBeta,
                k => panic!("unknown version type {}", k),
            },

            k => panic!("invaild version Value type {}", k),
        };

        let url = obj["url"].as_str().unwrap().to_string();
        let version = Version { kind, url };

        versions.insert(id, version);
    }

    let env = Env::new(
        versions,
        manifest["latest"].as_object().unwrap()["release"]
            .as_str()
            .unwrap()
            .to_string(),
    );
}

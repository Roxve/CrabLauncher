use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
};

use reqwest;
use serde_json::{Map, Value};

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
    pub latest_release: String,
    pub latest_snapshot: String,
}

impl Env {
    pub fn new(
        versions: HashMap<String, Version>,
        latest_release: String,
        latest_snapshot: String,
    ) -> Self {
        Self {
            versions,
            latest_release,
            latest_snapshot,
        }
    }

    pub fn from_manifest(manifest: Map<String, Value>) -> Self {
        let mut versions: HashMap<String, Version> = HashMap::new();

        let manifest_versions = manifest["versions"].as_array().unwrap();

        for obj in manifest_versions {
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
        let manifest_latest = manifest["latest"].as_object().unwrap();

        let manifest_latest_release = manifest_latest["release"].as_str().unwrap().to_string();
        let manifest_latest_snapshot = manifest_latest["snapshot"].as_str().unwrap().to_string();

        Env::new(versions, manifest_latest_release, manifest_latest_snapshot)
    }
}

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
    dbg!(&env);
}

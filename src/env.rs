use crate::instances::{init_profile, Profile, Version, VersionKind};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Env {
    pub versions: HashMap<String, Version>,
    pub latest_release: String,
    pub latest_snapshot: String,

    pub profiles: Vec<Profile>,
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
            profiles: init_profile(),
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
            let version = Version {
                id: id.clone(),
                kind,
                url,
            };

            versions.insert(id, version);
        }
        let manifest_latest = manifest["latest"].as_object().unwrap();

        let manifest_latest_release = manifest_latest["release"].as_str().unwrap().to_string();
        let manifest_latest_snapshot = manifest_latest["snapshot"].as_str().unwrap().to_string();

        Env::new(versions, manifest_latest_release, manifest_latest_snapshot)
    }
}

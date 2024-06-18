use crate::LAUNCHER_DIR;

use serde::{de::Visitor, Deserialize};

use std::fs;

#[derive(Debug)]
pub enum VersionKind {
    Release,
    Snapshot,
    OldAlpha,
    OldBeta,
}

impl<'de> Deserialize<'de> for VersionKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VersionVistor;
        impl<'de> Visitor<'de> for VersionVistor {
            type Value = VersionKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "old_alpha, old_beta, release, or snapshot")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "old_alpha" => Ok(VersionKind::OldAlpha),
                    "old_beta" => Ok(VersionKind::OldBeta),
                    "release" => Ok(VersionKind::Release),
                    "snapshot" => Ok(VersionKind::Snapshot),
                    _ => Err(E::custom(format!("invaild value for VersionKind {}", v))),
                }
            }
        }

        deserializer.deserialize_str(VersionVistor)
    }
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: VersionKind,
    pub url: String,
    #[allow(unused)]
    time: String,
    #[allow(unused)]
    #[serde(rename = "releaseTime")]
    release_time: String,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

impl Manifest {
    pub fn init_manifest() -> Self {
        let path = format!("{LAUNCHER_DIR}version_manifest.json");
        // download version info
        let res =
            reqwest::blocking::get("https://launchermeta.mojang.com/mc/game/version_manifest.json");
        // if offline use pre-downloaded file
        let manifest = if res.is_ok() {
            let res = res.unwrap();

            let buffer = res.text().unwrap();
            let bytes = buffer.as_bytes();

            fs::write(path, bytes).expect("failed writing file version_manifest.json");

            serde_json::from_str(buffer.as_str())
                .expect("failed reading file version_manifest.json")
        } else {
            let buffer = fs::read_to_string(path)
                .expect("opened version_manifest.json, but failed reading it");

            serde_json::from_str(buffer.as_str())
                .expect("failed parsing file version_manifest.json")
        };

        manifest
    }
}

use std::{
    fs::{self, File},
    io::{Read, Write},
};

use serde::{de::Visitor, Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub version: String,
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

pub fn init_profile() -> Vec<Profile> {
    let file = File::open("launcher/profiles.json");

    let mut file = if file.is_err() {
        let mut file = File::create_new("launcher/profiles.json").unwrap();
        file.write("{}".as_bytes()).unwrap();

        file
    } else {
        file.unwrap()
    };

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)
        .expect("failed to read launcher/profiles.json");

    if buffer.as_str() == "{}" {
        return Vec::new();
    }

    let profiles: Vec<Profile> = serde_json::from_str(buffer.as_str()).unwrap();

    return profiles;
}

pub fn write_profile(profile: &Profile) {
    let mut profiles = init_profile();
    profiles.push(profile.to_owned());

    let str = serde_json::to_string(&profiles).unwrap();

    fs::write("launcher/profiles.json", str).unwrap();
}

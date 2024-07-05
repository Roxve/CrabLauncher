use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{config::Config, json::client::Client, LAUNCHER_DIR, PROFILES_DIR};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Profile {
    pub name: String,
    pub version: String,

    #[serde(skip)]
    pub config: Option<Config>,
}

pub fn init_profile() -> Vec<Profile> {
    let file = File::open("launcher/profiles.json");

    let mut file = if file.is_err() {
        let mut file = File::create_new("launcher/profiles.json").unwrap();
        file.write("[]".as_bytes()).unwrap();

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

    let mut profiles: Vec<Profile> = serde_json::from_str(buffer.as_str()).unwrap();

    for profile in &mut profiles {
        let profile_spec_config = format!("{PROFILES_DIR}/{}/config.json", profile.name);
        let profile_spec_config = Path::new(&profile_spec_config);

        let config = if profile_spec_config.exists() {
            let config = fs::read_to_string(profile_spec_config).unwrap();
            Some(serde_json::from_str(&config).unwrap())
        } else {
            None
        };

        profile.config = config;
    }

    profiles
}

pub fn write_profile(profile: &Profile) {
    let mut profiles = init_profile();
    profiles.push(profile.to_owned());

    let str = serde_json::to_string(&profiles).unwrap();

    fs::write("launcher/profiles.json", str).unwrap();
}

pub fn write_profiles(profiles: &Vec<Profile>) {
    let profiles_json =
        serde_json::to_string_pretty(profiles).expect("Failed to serialize profiles");

    let profiles_path = format!("{}/profiles.json", LAUNCHER_DIR);
    fs::write(profiles_path, profiles_json).expect("Failed to write profiles to file");
}

pub fn read_profile_setup(name: String) -> Client {
    let mut client: Client = serde_json::from_str(
        fs::read_to_string(format!("{PROFILES_DIR}{0}/{0}.json", name))
            .unwrap()
            .as_str(),
    )
    .unwrap();

    client.profile_name = Some(name);
    client
}

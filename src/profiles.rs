use std::{
    fs::{self, File},
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::{json::client::Client, PROFILES_DIR};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Profile {
    pub name: String,
    pub version: String,
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

    let profiles: Vec<Profile> = serde_json::from_str(buffer.as_str()).unwrap();

    return profiles;
}

pub fn write_profile(profile: &Profile) {
    let mut profiles = init_profile();
    profiles.push(profile.to_owned());

    let str = serde_json::to_string(&profiles).unwrap();

    fs::write("launcher/profiles.json", str).unwrap();
}

pub fn write_profiles(profiles: &Vec<Profile>) {
    let str = serde_json::to_string(profiles).unwrap();

    fs::write("launcher/profiles.json", str).unwrap();
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

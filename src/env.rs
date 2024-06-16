use crate::instances::{init_profile, write_profile, Manifest, Profile};
use std::fs;

#[derive(Debug)]
pub struct Env {
    pub profiles: Vec<Profile>,
    pub manifest: Manifest,
}

impl Env {
    pub fn from_manifest(manifest: Manifest) -> Self {
        Self {
            manifest: manifest,
            profiles: init_profile(),
        }
    }

    pub fn get_url(&self, ver: &String) -> &String {
        &self
            .manifest
            .versions
            .iter()
            .find(|v| &v.id == ver)
            .unwrap()
            .url
    }

    pub fn add_profile(&mut self, profile: Profile) {
        let profile_dir = &format!("launcher/profiles/{}", &profile.name);
        fs::create_dir_all(profile_dir).unwrap();

        // downloading version json
        let url = self.get_url(&profile.version);

        let res = reqwest::blocking::get(url).expect("failed to download version json file from url, make sure you are connected to the internet");
        let text = res.text().unwrap();
        // writing json
        fs::write(format!("{profile_dir}/{}.json", &profile.name), text).unwrap();

        write_profile(&profile);

        self.profiles.push(profile);
    }
}

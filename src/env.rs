use crate::client::get_req_libs;
use crate::config::Config;
use crate::json::manifest::Manifest;
use crate::profiles::{init_profile, read_profile_setup, write_profile, write_profiles, Profile};
use crate::{ASSETS_DIR, PROFILES_DIR};

use std::fs;
use std::process::Command;

#[derive(Debug)]
pub struct Env {
    pub profiles: Vec<Profile>,
    pub manifest: Manifest,
    pub config: Config,
}

impl Env {
    pub fn from_manifest(manifest: Manifest) -> Self {
        Self {
            manifest: manifest,
            profiles: init_profile(),
            config: Config::default(),
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
        if self.profiles.contains(&profile) {
            return;
        }

        if self
            .profiles
            .iter()
            .find(|x| x.name == profile.name)
            .is_some()
        {
            return;
        }

        let profile_dir = &format!("launcher/profiles/{}", &profile.name);
        fs::create_dir_all(profile_dir).unwrap();

        // downloading version json
        let url = self.get_url(&profile.version);

        let res = reqwest::blocking::get(url).expect("failed to download version json file from url, make sure you are connected to the internet");
        let text = res.text().unwrap();
        // writing json
        fs::write(format!("{profile_dir}/{}.json", &profile.name), text).unwrap();

        write_profile(&profile);

        self.profiles.push(profile.clone());

        println!(
            "created new profile {} with version {}",
            profile.name, profile.version
        );
    }

    pub fn del_profile(&mut self, name: String) {
        let mut found = false;

        for (index, profile) in self.profiles.iter().enumerate() {
            if &profile.name == &name {
                self.profiles.remove(index);
                found = true;
                break;
            }
        }

        if !found {
            return;
        }

        fs::remove_dir_all(format!("{PROFILES_DIR}{}", name)).unwrap();
        write_profiles(&self.profiles);

        println!("removed profile {name}!");
    }

    pub fn run(&self, name: String) {
        let client = read_profile_setup(name.clone());
        let profile = self.profiles.iter().find(|x| &x.name == &name);

        if profile.is_none() {
            return;
        }
        let path = format!("{PROFILES_DIR}{name}");
        let profile = profile.unwrap();

        let libs: Vec<String> = get_req_libs(&client);
        let classpath = libs.join(":");

        Command::new(&self.config.java)
            .arg(format!("-Xmx{}M", self.config.max_ram))
            .arg(format!("-Xms{}M", self.config.min_ram))
            .arg(format!("-Djava.library.path={path}/.natives"))
            .arg("-cp")
            .arg(format!("{classpath}:{path}/{name}.jar"))
            .arg(client.main_class)
            .arg("--accessToken")
            .arg(&self.config.access_token)
            .arg("--username")
            .arg(&self.config.username)
            .arg("--version")
            .arg(&profile.version)
            .arg("--gameDir")
            .arg(path)
            .arg("--assetsDir")
            .arg(ASSETS_DIR)
            .arg("--assetIndex")
            .arg(client.assets)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

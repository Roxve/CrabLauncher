use crate::client::get_req_libs;
use crate::config::Config;
use crate::json::manifest::Manifest;
use crate::profiles::{init_profile, read_profile_setup, write_profile, write_profiles, Profile};
use crate::{ASSETS_DIR, PROFILES_DIR};

use std::fs;
use std::io::stdin;
use std::process::Command;

#[derive(Debug)]
pub struct Env {
    pub profiles: Vec<Profile>,
    pub manifest: Manifest,
    pub config: Config,
}

impl Env {
    pub fn new(manifest: Manifest, config: Config) -> Self {
        Self {
            manifest,
            config,
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

    // TODO!: ADD ERRORS
    pub fn edit_profile_entry(&mut self, name: String, entry: String, value: Option<String>) {
        let profile = self.profiles.iter_mut().find(|x| &x.name == &name);

        if profile.is_none() {
            panic!("couldnt find profile with name {name}");
        }

        let profile = profile.unwrap();

        let config = if profile.config.is_some() {
            profile.config.as_mut().unwrap()
        } else {
            &mut self.config
        };

        let value = value.unwrap_or_default();

        match entry.as_str() {
            "config.access_token" => {
                config.access_token = value;
            }

            "config.username" => {
                config.username = value;
            }

            "config.current_java_path" => {
                config.current_java_path = value;
            }

            "config.max_ram" => {
                config.max_ram = value.parse().unwrap();
            }

            "config.min_ram" => {
                config.min_ram = value.parse().unwrap();
            }

            "java" => {
                println!("available java installations:");

                for (index, java) in config.java_list.iter().enumerate() {
                    println!(
                        "{index}:\n\tpath: {}\n\tversion: {}",
                        java.path, java.version
                    );
                }

                let mut buffer = String::new();

                println!("enter the index of the java installation you want to use: ");
                stdin().read_line(&mut buffer).unwrap();

                let index: usize = buffer.trim().parse().unwrap();

                let java = config.java_list.get(index);

                if java.is_none() {
                    panic!("couldnt find java installation with index {index}");
                }

                let java = java.unwrap().clone();

                config.current_java_path = java.path.clone();
            }

            _ => panic!("couldnt find entry {entry} in profile {name}"),
        }

        let path = format!("{PROFILES_DIR}{name}/config.json");

        fs::write(path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
    }

    pub fn run(&self, name: String) {
        let client = read_profile_setup(name.clone());
        let profile = self.profiles.iter().find(|x| &x.name == &name);

        if profile.is_none() {
            return;
        }

        let path = format!("{PROFILES_DIR}{name}");
        let profile = profile.unwrap();

        let config = if profile.config.is_some() {
            profile.config.as_ref().unwrap()
        } else {
            &self.config
        };

        let libs: Vec<String> = get_req_libs(&client);
        let classpath = libs.join(":");

        Command::new(&config.current_java_path)
            .arg(format!("-Xmx{}M", config.max_ram))
            .arg(format!("-Xms{}M", config.min_ram))
            .arg(format!("-Djava.library.path={path}/.natives"))
            .arg("-cp")
            .arg(format!("{classpath}:{path}/{name}.jar"))
            .arg(client.main_class)
            .arg("--accessToken")
            .arg(&config.access_token)
            .arg("--username")
            .arg(&config.username)
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

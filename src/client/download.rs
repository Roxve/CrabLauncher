use std::{fs, path::Path};

use crate::{
    error::Error,
    json::client::{Client, Download, Index},
    ASSETS_DIR, LIB_DIR, PROFILES_DIR,
};

impl Download {
    fn download(&self) -> Result<Option<reqwest::blocking::Response>, Error> {
        // lib else assets
        if self.path.is_some() {
            let path = LIB_DIR.to_owned() + self.path.as_ref().unwrap();
            let path = Path::new(&path);
            if path.exists() {
                return Ok(None);
            }

            let res = reqwest::blocking::get(&self.url).unwrap();

            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, res.bytes().unwrap()).unwrap();

            Ok(None)
        } else if self.id.is_some() {
            let indexes_path = format!("{ASSETS_DIR}indexes/{}.json", self.id.as_ref().unwrap());
            let indexes_path = Path::new(&indexes_path);

            let res = reqwest::blocking::get(&self.url).unwrap();
            let bytes = res.bytes().unwrap();
            fs::create_dir_all(indexes_path.parent().unwrap()).unwrap();
            fs::write(indexes_path, bytes.clone()).unwrap();

            // downloading objects
            let index: Index = serde_json::from_slice(&bytes.to_vec()).unwrap();
            let objects = index.objects;

            for (_, object) in objects {
                let dir = &object.hash[0..2];
                let dir_all = format!("{ASSETS_DIR}objects/{}", dir);

                let path = &format!("{dir_all}/{}", object.hash);
                let path = Path::new(path);
                if path.exists() {
                    continue;
                }

                fs::create_dir_all(&dir_all).unwrap();
                let res = reqwest::blocking::get(format!(
                    "https://resources.download.minecraft.net/{dir}/{}",
                    object.hash
                ))
                .unwrap();

                fs::write(path, res.bytes().unwrap()).unwrap();
            }
            Ok(None)
        } else {
            // just download!
            Ok(Some(reqwest::blocking::get(self.url.clone()).unwrap()))
        }
    }
}

pub fn download(client: Client) {
    client.asset_index.download().unwrap();

    for lib in client.libraries {
        if lib.rules.is_some() {
            let rules = lib.rules.unwrap();

            let mut is_allowed = true;
            for rule in rules {
                is_allowed = rule.is_allowed();

                if !is_allowed {
                    break;
                }
            }

            if !is_allowed {
                continue;
            }
        }
        lib.downloads.artifact.download().unwrap();
    }

    // client path
    let name = client.profile_name.unwrap();

    let path = format!("{PROFILES_DIR}{name}/{name}.jar");
    let path = Path::new(&path);

    if path.exists() {
        return;
    }

    // downloading client.jar
    let res = client.downloads.client.download().unwrap().unwrap();

    fs::write(path, res.bytes().unwrap()).unwrap();
}

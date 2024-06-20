use std::{fs, io::Cursor, path::Path};

use zip::{result::ZipError, ZipArchive};

use crate::{
    client::rule::is_allowed,
    error::Error,
    json::client::{Client, Download, Index},
    ASSETS_DIR, LIB_DIR, PROFILES_DIR,
};

impl Download {
    fn download(&self, path: String) -> Result<Option<reqwest::blocking::Response>, Error> {
        if &path == &String::new() {
            return Ok(Some(reqwest::blocking::get(self.url.clone()).unwrap()));
        }

        // lib else assets
        if self.path.is_some() {
            let path = path + self.path.as_ref().unwrap();
            let path = Path::new(&path);
            if path.exists() {
                return Ok(None);
            }

            let res = reqwest::blocking::get(&self.url).unwrap();

            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, res.bytes().unwrap()).unwrap();

            Ok(None)

        // todo move objects downloads somewhere else
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
            let path = Path::new(&path);
            if path.exists() {
                return Ok(None);
            }

            let res = reqwest::blocking::get(self.url.clone()).unwrap();

            fs::write(path, res.bytes().unwrap()).unwrap();

            Ok(None)
        }
    }
}

pub fn download(client: Client) {
    client
        .asset_index
        .download(format!("{ASSETS_DIR}indexes\\{}.json", client.assets))
        .unwrap();

    for lib in client.libraries {
        if lib.rules.is_some() {
            let rules = lib.rules.unwrap();

            if !is_allowed(&rules) {
                continue;
            }
        }

        if lib.downloads.artifact.is_some() {
            lib.downloads
                .artifact
                .as_ref()
                .unwrap()
                .download(LIB_DIR.to_string())
                .unwrap();
        }

        if lib.natives.is_some() {
            let natives = lib.natives.unwrap();
            for (os, native) in natives {
                if os == crate::OS {
                    let classifiers = lib.downloads.classifiers.as_ref().unwrap();
                    let classifier = classifiers.get(&native).unwrap();

                    if lib.extract.is_none() {
                        classifier.download(LIB_DIR.to_string()).unwrap();
                    } else {
                        let res = classifier.download(String::new()).unwrap().unwrap();

                        extract(
                            res.bytes().unwrap().to_vec(),
                            format!(
                                "{PROFILES_DIR}{}/.natives",
                                client.profile_name.as_ref().unwrap()
                            ),
                            lib.extract.unwrap().exclude,
                        )
                        .unwrap();
                    }

                    break;
                }
            }
        }
    }

    // client path
    let name = client.profile_name.unwrap();

    let path = format!("{PROFILES_DIR}{name}/{name}.jar");

    // downloading client.jar
    client.downloads.client.download(path).unwrap();
}

fn extract(jar: Vec<u8>, output: String, exclude: Option<Vec<String>>) -> Result<(), ZipError> {
    let exclude = exclude.unwrap_or_default();

    let reader = Cursor::new(jar);
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let file_path = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        if exclude.contains(&file_path.to_str().unwrap().to_string())
            || exclude.contains(&(file_path.parent().unwrap().to_str().unwrap().to_string() + "/"))
        {
            continue;
        }

        let output = Path::new(&output).join(file_path.clone());

        if file.name().ends_with('/') {
            fs::create_dir_all(output).unwrap();
        } else {
            if let Some(p) = output.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = std::fs::File::create(&output)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

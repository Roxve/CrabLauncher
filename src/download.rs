use std::{fs, path::Path};

use crate::{
    error::Error,
    setup::{Download, Index, Setup},
    ASSETS_DIR, LIB_DIR,
};

impl Download {
    fn download(&self) -> Result<(), Error> {
        // lib else assets
        if self.path.is_some() {
            let path = LIB_DIR.to_owned() + self.path.as_ref().unwrap();
            let path = Path::new(&path);
            if path.exists() {
                return Ok(());
            }

            let res = reqwest::blocking::get(&self.url).unwrap();

            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, res.bytes().unwrap()).unwrap();
        } else if self.id.is_some() {
            let indexs_path = format!("{ASSETS_DIR}indexs/{}.json", self.id.as_ref().unwrap());
            let indexs_path = Path::new(&indexs_path);

            let res = reqwest::blocking::get(&self.url).unwrap();
            let bytes = res.bytes().unwrap();
            fs::create_dir_all(indexs_path.parent().unwrap()).unwrap();
            fs::write(indexs_path, bytes.clone()).unwrap();

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
        }
        Ok(())
    }
}

pub fn download(client: Setup) {
    client.asset_index.download().unwrap();
    for lib in client.libraries {
        lib.downloads.artifact.download().unwrap();
    }
}

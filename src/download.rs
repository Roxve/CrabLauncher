use std::{fs, path::Path};

use crate::{
    error::Error,
    setup::{Download, Setup},
    ASSETS_DIR, LIB_DIR,
};

impl Download {
    fn download(&self) -> Result<(), Error> {
        // lib else assets
        if self.path.is_some() {
            let path = LIB_DIR.to_owned() + self.path.as_ref().unwrap();
            let path = Path::new(&path);
            if path.exists() {
                dbg!(path);
                return Ok(());
            }

            let res = reqwest::blocking::get(&self.url).unwrap();

            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, res.bytes().unwrap()).unwrap();
        } else if self.id.is_some() {
            let indexs_path = format!("{ASSETS_DIR}indexs/{}.json", self.id.as_ref().unwrap());
            let indexs_path = Path::new(&indexs_path);
            if indexs_path.exists() {
                dbg!(indexs_path);
                return Ok(());
            }
            let res = reqwest::blocking::get(&self.url).unwrap();

            fs::create_dir_all(indexs_path.parent().unwrap()).unwrap();
            fs::write(indexs_path, res.bytes().unwrap()).unwrap();
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

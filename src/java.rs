use std::{
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

use rust_search::SearchBuilder;

use crate::{json::client::OsName, OS};

pub struct JavaInstallation {
    path: String,
    version: String,
}

pub fn find() -> Vec<String> {
    let search = SearchBuilder::default();
    if OS == OsName::Linux {
        let paths: Vec<PathBuf> = search
            .location("/")
            .search_input("java")
            .strict()
            .build()
            .map(|x| Path::new(&x).to_path_buf())
            .collect();

        paths
            .iter()
            .filter(|x| {
                x.is_file()
                    && x.metadata().unwrap().permissions().mode() & 0o111 != 0 // check if an exe
                    && x.file_name().unwrap_or_default() == "java"
            }) // search has some problems so i had to do this here
            .map(|x| x.to_str().unwrap().to_string())
            .collect()
    } else {
        todo!()
    }
}

pub fn list() -> Vec<JavaInstallation> {
    let paths = find();

    for path in paths {
        let version = Command::new(path)
            .arg("-version")
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap()
            .stdout;
        let version = String::from_utf8(version).unwrap();
        todo!()
    }
    todo!()
}

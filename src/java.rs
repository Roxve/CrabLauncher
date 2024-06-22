use std::collections::VecDeque;

use std::{
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

use regex::Regex;
use rust_search::SearchBuilder;

use crate::{json::client::OsName, OS};

pub struct JavaInstallation {
    pub path: String,
    pub version: String,
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
    let mut list = Vec::new();

    for path in paths {
        let version = Command::new(&path).arg("-version").output().unwrap();

        let version = String::from_utf8(version.stderr).unwrap();

        let regex = Regex::new(r#"version "(\d+\.\d+\.\d+)_?(\d+)?""#).unwrap(); // ^"\d+(\.\d+)*"$

        let captures = regex.captures(&version).unwrap();

        let version = captures[0]
            .to_string()
            .replace("version ", "")
            .replace("\"", "");

        list.push(JavaInstallation { path, version });
    }

    sort_by_version(&mut list);
    return list;
}

fn get_version(version: &String) -> u32 {
    let version: Vec<u8> = version.bytes().collect();

    let mut res = 0;
    let mut index = 0;

    for n in (0..version.len()).rev() {
        if version[n] != ('.' as u8) && version[n] != ('_' as u8) {
            res += ((version[n] - '0' as u8) as u32) * 10_u32.pow(index);
            index += 1;
        }
    }

    return res;
}

fn sort_by_version(list: &mut Vec<JavaInstallation>) {
    list.sort_by(|a, b| {
        let a: Vec<&str> = a.version.split('.').collect();
        let b: Vec<&str> = b.version.split('.').collect();

        let a: f32 = a[2].replace("_", ".").parse().unwrap();
        let b: f32 = b[2].replace("_", ".").parse().unwrap();
        b.partial_cmp(&a).unwrap()
    });

    list.sort_by(|a, b| {
        let a: Vec<&str> = a.version.split('.').collect();
        let b: Vec<&str> = b.version.split('.').collect();

        let a: u32 = a[1].parse().unwrap();
        let b: u32 = b[1].parse().unwrap();
        b.cmp(&a)
    });

    list.sort_by(|a, b| {
        let a: Vec<&str> = a.version.split('.').collect();
        let b: Vec<&str> = b.version.split('.').collect();

        let a: u32 = a[0].parse().unwrap();
        let b: u32 = b[0].parse().unwrap();
        b.cmp(&a)
    });
}

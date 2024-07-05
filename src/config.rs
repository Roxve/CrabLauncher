use clap::Args;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::java::{self, JavaInstallation};
use crate::LAUNCHER_DIR;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub min_ram: i32,
    pub max_ram: i32,

    pub width: i32,
    pub height: i32,

    pub username: String,
    pub access_token: String,

    pub current_java_path: String,

    pub java_list: Vec<JavaInstallation>,
}

impl Default for Config {
    fn default() -> Self {
        let java_list = java::list();

        Self {
            min_ram: 512,
            max_ram: 2048,
            width: 854,
            height: 480,
            username: String::from("dev"),
            access_token: String::from("0"),
            current_java_path: java_list[0].path.clone(),
            java_list,
        }
    }
}

impl Config {
    pub fn init_config() -> Self {
        let path = format!("{LAUNCHER_DIR}/config.json");
        let path = Path::new(&path);

        let config = if !path.exists() {
            let config = Self::default();
            let file = File::create(path).unwrap();

            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &config).unwrap();
            config
        } else {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };

        config
    }
}

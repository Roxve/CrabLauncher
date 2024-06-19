use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum OsName {
    Linux,
    Windows,
    Osx,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86_64,
    X86,
    ARM64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Os {
    pub name: Option<OsName>,
    pub version: Option<String>,
    pub arch: Option<Arch>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rule {
    pub action: String,
    pub features: Option<Value>,
    pub os: Option<Os>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ArgValue {
    Value(String),
    Values(Vec<String>),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Argument {
    Arg(String),
    Rule { rules: Vec<Rule>, value: ArgValue },
}
#[derive(Deserialize, Debug)]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Download {
    pub id: Option<String>,
    pub path: Option<String>,
    pub sha1: String,
    pub size: i32,
    #[serde(rename = "totalSize")]
    pub total_size: Option<i32>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub client: Download,
    pub client_mappings: Download,
    pub server: Download,
    pub server_mappings: Download,
}

#[derive(Debug, Deserialize)]
pub struct JavaVersion {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LibraryDownload {
    pub artifact: Download,
    pub classifiers: Option<HashMap<String, Download>>,
}

#[derive(Debug, Deserialize)]
pub struct Library {
    pub downloads: LibraryDownload,
    pub name: String,
    pub natives: Option<HashMap<OsName, String>>,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub arguments: Arguments,
    #[serde(rename = "assetIndex")]
    pub asset_index: Download,

    pub assets: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: i32,

    pub downloads: Downloads,
    pub id: String,

    #[serde(rename = "javaVersion")]
    pub java_version: JavaVersion,

    pub libraries: Vec<Library>,
    #[serde(rename = "mainClass")]
    pub main_class: String,

    pub profile_name: Option<String>,
}

// assets
#[derive(Deserialize, Debug)]
pub struct Object {
    pub hash: String,
    pub size: i32,
}

#[derive(Deserialize, Debug)]
pub struct Index {
    pub objects: HashMap<String, Object>,
}

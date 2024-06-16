use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Os {
    pub name: Option<String>,
    pub arch: Option<String>,
}

#[derive(Deserialize, Debug)]
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

pub struct Setup {
    pub arguments: Arguments,
}

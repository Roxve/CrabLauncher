#[derive(Debug)]
pub struct Config {
    pub min_ram: i32,
    pub max_ram: i32,

    pub width: i32,
    pub height: i32,

    pub username: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min_ram: 512,
            max_ram: 2048,
            width: 854,
            height: 480,
            username: String::from("dev"),
        }
    }
}

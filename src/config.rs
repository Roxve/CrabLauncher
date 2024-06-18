pub struct Config {
    pub min_ram: i32,
    pub max_ram: i32,

    pub width: i32,
    pub height: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min_ram: 512,
            max_ram: 2000,
            width: 854,
            height: 480,
        }
    }
}

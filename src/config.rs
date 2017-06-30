//
//
//
//
//
//

extern crate toml;

use std::fs::File;
use std::path::Path;
use std::io::Read;

pub static DEFAULT_PATH: &'static str = ".icingacli";
pub static DEFAULT_PORT: u16 = 5665;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub server: String,
    pub port: i64,
    pub user: String,
    pub password: Option<String>,
}

impl Config {
    pub fn read_config(path: &Path) -> Result<Config, toml::de::Error> {
        let mut data = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read string");
        let c: Config = toml::from_str(&data).unwrap();
        Ok(c)
    }
}

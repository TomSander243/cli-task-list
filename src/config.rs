use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub user_dir: PathBuf,
}

pub fn read_config(path: &str) -> Config {
    if Path::new(path).exists() {
        let mut file = File::open(path).expect("Unable to open config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read config file");
        serde_json::from_str(&contents).expect("Unable to parse config file")
    } else {
        let config = Config {
            user_dir: PathBuf::from("./taskgroups"),
        };
        write_config(path, &config);
        config
    }
}

pub fn write_config(path: &str, config: &Config) {
    let contents = serde_json::to_string_pretty(config).expect("Unable to serialize config");
    fs::write(path, contents).expect("Unable to write config file");
}

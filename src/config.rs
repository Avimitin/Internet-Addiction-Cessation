use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[allow(dead_code)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    pub duration: Duration,
    pub block_domains: HashMap<String, Vec<String>>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Duration {
    pub start: String,
    pub end: String,
}

impl Config {
    // new read from configuration file from given path and parse it into Config struct.
    // Panic when no file found or fail to parse the contents into the Config struct.
    pub fn new(file_path: &str) -> Config {
        let path = Path::new(file_path);
        let contents = std::fs::read(path).unwrap();
        let contents = String::from_utf8(contents).expect("invalid file character");
        let config = toml::from_str(&contents)
            .expect(format!("failed to read from config: {}", contents).as_str());

        return config;
    }

    pub fn build_domains(&self) -> String {
        let mut s = String::new();
        for (k, v) in &self.block_domains {
            for prefix in v {
                if prefix == "@" {
                    s.push_str(format!("{domain}\n", domain = k).as_str());
                } else {
                    s.push_str(format!("{prefix}.{domain}\n", prefix = prefix, domain = k).as_str());
                }
            }
        }

        return s;
    }
}

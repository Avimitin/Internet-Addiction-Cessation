use serde::Deserialize;
use std::collections::HashMap;
use anyhow::{Context, Result};

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
    pub fn new(file_path: &str) -> Result<Config> {
        let contents = std::fs::read_to_string(file_path)
            .with_context(||{format!("Read config file {} fail", file_path)})?;
        let config = toml::from_str(&contents)
            .with_context(||{format!("failed to read from config: {}", contents)})?;

        return Ok(config);
    }

    pub fn build_domains(&self) -> Vec<String> {
        let mut s = Vec::new();
        for (k, v) in &self.block_domains {
            for prefix in v {
                if prefix == "@" {
                    s.push(format!("{domain}", domain = k));
                } else {
                    s.push(format!("{prefix}.{domain}", prefix = prefix, domain = k));
                }
            }
        }

        return s;
    }

    pub fn end_when(&self) -> Option<(u32, u32)> {
        let v: Vec<&str> = self.duration.end.rsplit(':').collect();
        if v.len() < 2 {
            return None
        }
        let hour: u32 = v[1].parse().unwrap_or(0);
        let min: u32 = v[0].parse().unwrap_or(0);
        if hour == 0 && min == 0 {
            return None;
        }
        Some((hour,min))
    }
}

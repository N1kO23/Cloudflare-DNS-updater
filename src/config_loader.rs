use serde::Deserialize;
use std::error::Error;
use std::fs;

/// The structure of config
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    /// The threshold of how often to update the DNS records in seconds
    pub update_threshold: u64,
    /// The list of keys to update
    pub keys: Vec<Key>,
}

/// The structure of the key for zones
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Key {
    /// The authentication key for the Cloudflare API
    pub auth_key: String,
    /// The list of zones to update
    pub zones: Vec<Zone>,
}

/// The structure of a zone
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Zone {
    /// The zone ID
    pub zone_id: String,
    /// The list of A records to update
    pub a_records: Vec<String>,
    /// The list of AAAA records to update
    pub aaaa_records: Vec<String>,
}

/// Loads a config from file and returns it. If load fails, an error is thrown.
///
/// # Arguments
/// * `path` - Path to a custom config file, if unspecified, the config is loaded from root directory.
///
/// # Returns
/// * `Config` - The loaded config
///
/// # Errors
/// * `std::io::Error` - If the config file cannot be read
/// * `serde_json::Error` - If the config file cannot be parsed
pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let str: String;
    str = fs::read_to_string(path)?.parse()?;
    let conf: Config = serde_json::from_str(&str)?;
    Ok(conf)
}

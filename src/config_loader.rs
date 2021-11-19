use serde::Deserialize;
use std::error::Error;
use std::fs;

/// The structure of config
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
  /// The authentication key for the Cloudflare API
  pub auth_key: String,
  /// The threshold of how often to update the DNS records in seconds
  pub update_threshold: u64,
  /// The list of zones to update
  pub zones: Vec<Zone>,
}

/// The structure of a zone
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Zone {
  /// The zone ID
  pub zone_id: String,
  /// The list of records to update
  pub records: Vec<String>,
}

/// Loads a config from file and returns it. If load fails, an error is thrown.
/// # Arguments
/// * `path` - Path to a custom config file, if unspecified, the config is loaded from root directory.
/// # Returns
/// * `Config` - The loaded config
/// # Errors
/// * `std::io::Error` - If the config file cannot be read
/// * `serde_json::Error` - If the config file cannot be parsed
pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
  let str: String;
  str = fs::read_to_string(path).unwrap().parse().unwrap();
  let conf: Config = serde_json::from_str(&str).unwrap();
  Ok(conf)
}

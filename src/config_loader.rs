use serde::Deserialize;
use std::error::Error;
use std::fs;

/// The structure of config
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
  /// The authentication key for the Cloudflare API
  pub auth_key: String,
  /// The list of zones to update
  pub zones: Vec<Zone>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Zone {
  /// The zone ID
  pub zone_id: String,
  /// The list of records to update
  pub records: Vec<String>,
}

/// Loads a config from file and returns it. If load fails, an error is thrown.
/// ### Arguments
/// * `path` - Path to a custom config file, if unspecified, the config is loaded from root directory.
pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
  let str: String;
  str = fs::read_to_string(path).unwrap().parse().unwrap();
  let conf: Config = serde_json::from_str(&str).unwrap();
  Ok(conf)
}

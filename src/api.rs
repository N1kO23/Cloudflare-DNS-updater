use serde::{Deserialize, Serialize};

/// Struct for the current IP address
#[derive(Serialize, Deserialize)]
pub struct CurrentIP {
  /// The current IP address
  pub ip: String,
}

/// Struct for the Cloudflares response
#[derive(Serialize, Deserialize, Debug)]
pub struct CloudFlareResult {
  /// The Cloudflare response containing the array of records
  pub result: Vec<DNSRecordResult>,
}
/// The struct for the DNS record
#[derive(Serialize, Deserialize, Debug)]
pub struct DNSRecordResult {
  /// The DNS record ID
  pub id: String,
  /// The DNS record name
  pub name: String,
  /// The DNS record IP address
  pub content: String,
  /// Is the DNS record locked?
  pub locked: bool,
  /// Is the DNS record proxied?
  pub proxied: bool,
  /// The DNS record ttl
  pub ttl: u32,
  /// The DNS record zone ID
  pub zone_id: String,
  /// The DNS record modified date
  pub modified_on: String,
}

/// Struct for the record update template
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRecord {
  /// The DNS record type
  pub r#type: String,
  /// The DNS record name
  pub name: String,
  /// The DNS record IP address
  pub content: String,
  /// The DNS record ttl
  pub ttl: u32,
  /// Is the DNS record proxied?
  pub proxied: bool,
}

const IP_ADDRESS_URL: &str = "https://api.ipify.org?format=json";
const CF_BASE_URL: &str = "https://api.cloudflare.com/client/v4/zones/";

pub async fn get_current_ip() -> Result<String, reqwest::Error> {
  let response = reqwest::get(IP_ADDRESS_URL).await?;
  let cur_ip: CurrentIP = response.json().await?;
  Ok(cur_ip.ip)
}

pub async fn get_record_ip(
  records: &Vec<String>,
  zone: &str,
  auth_key: &str,
) -> Result<CloudFlareResult, Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let url = format!("{}{}/dns_records?type=A", CF_BASE_URL, zone);
  let res = client
    .get(url)
    .header("Authorization", format!("Bearer {}", auth_key))
    .send()
    .await?
    .text()
    .await?;
  let mut results: CloudFlareResult = serde_json::from_str(&res)?;
  results
    .result
    .retain(|record| records.contains(&record.name));
  Ok(results)
}

pub async fn update_record(
  record: &DNSRecordResult,
  ip: &str,
  auth_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let url = format!(
    "{}{}/dns_records/{}",
    CF_BASE_URL, record.zone_id, record.id
  );
  client
    .put(url)
    .header("Authorization", format!("Bearer {}", auth_key))
    .json(&UpdateRecord {
      r#type: "A".to_string(),
      name: record.name.to_string(),
      content: ip.to_string(),
      ttl: record.ttl,
      proxied: record.proxied,
    })
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
  Ok(())
}


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CurrentIP {
  pub ip: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloudFlareResult {
  pub result: Vec<DNSRecordResult>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DNSRecordResult {
  pub id: String,
  pub name: String,
  pub content: String,
  pub locked: bool,
  pub proxied: bool,
  pub ttl: u32,
  pub zone_id: String,
  pub modified_on: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateRecord {
  pub r#type: String,
  pub name: String,
  pub content: String,
  pub ttl: u32,
  pub proxied: bool,
}


const IP_ADDRESS_URL: &str = "https://api.ipify.org?format=json";
const CF_BASE_URL: &str = "https://api.cloudflare.com/client/v4/zones/"; 

pub async fn get_current_ip() -> Result<String, reqwest::Error> {
  let response = reqwest::get(IP_ADDRESS_URL).await?;
  let cur_ip: CurrentIP = response.json().await?;
  Ok(cur_ip.ip)
}

pub async fn get_record_ip(records: &[&'static str], zone: &str, auth_key: &str) -> Result<CloudFlareResult, Box<dyn std::error::Error>> {
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
    results.result.retain(|record| records.contains(&record.name.as_str()));
  Ok(results)
}

pub async fn update_record(record: &DNSRecordResult, ip: &str, auth_key: &str) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let url = format!("{}{}/dns_records/{}", CF_BASE_URL, record.zone_id, record.id);
  client
    .put(url)
    .header("Authorization", format!("Bearer {}", auth_key))
    .json(&UpdateRecord {
      r#type: "A".to_string(),
      name: record.name.to_string(),
      content: ip.to_string(),
      ttl: record.ttl,
      proxied: record.proxied
    })
    .send()
    .await.unwrap()
    .text()
    .await.unwrap();
  Ok(())
}
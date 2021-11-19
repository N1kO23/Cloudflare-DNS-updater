use api::CloudFlareResult;
use config_loader::Config;

mod api;
mod config_loader;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
  println!("Cloudflare IP updater v{}", VERSION);
  println!("Loading config...");
  let config = config_loader::load_config(handle_args().as_str()).expect("Failed to load config");
  loop {
    match check_and_update_ip(&config).await {
      Ok(()) => {}
      Err(e) => println!("Error: {}", e),
    }
    std::thread::sleep(std::time::Duration::from_secs(120));
  }
}

async fn check_and_update_ip(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  print!("\nGetting current IP address");
  let cur_ip = api::get_current_ip().await?;
  println!(" - {}\n", cur_ip);
  for z in 0..config.zones.len() {
    let record_ips: CloudFlareResult = api::get_record_ip(
      &config.zones[z].records,
      &config.zones[z].zone_id,
      &config.auth_key,
    )
    .await?;
    for i in 0..config.zones[z].records.len() {
      if cur_ip != record_ips.result[i].content {
        print!(
          "Updating record {} from {} to {}",
          record_ips.result[i].name, record_ips.result[i].content, cur_ip
        );
        match api::update_record(&record_ips.result[i], &cur_ip, &config.auth_key).await {
          Ok(()) => println!(" - Record updated"),
          Err(e) => println!(" - Error: {}", e),
        }
      } else {
        println!("Record {} is up to date", record_ips.result[i].name);
      }
    }
  }
  Ok(())
}

fn handle_args() -> String {
  let mut config_path: Option<&str> = None;
  let args: Vec<String> = std::env::args().collect();
  let mut index: usize = 0;
  while args.len() > index {
    let arg: &str = &args[index];
    match arg {
      "-c" => {
        index = index + 1;
        if index < args.len() {
          config_path = Some(&args[index]);
          index = index + 1;
          println!("Using custom config path: {:?}", config_path);
        }
      }
      _ => index = index + 1,
    }
  }
  return config_path.unwrap_or("config.json").to_string();
}

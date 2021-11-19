use api::CloudFlareResult;

mod api;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ZONE: &'static str = "zone_str";
const AUTH_KEY: &'static str = "auth_key"; 
const RECORDS: &'static [&'static str] = &["test.domain.com"];

#[tokio::main]
async fn main() {
    println!("Cloudflare IP updater v{}", VERSION);
    loop {
        match check_and_update_ip().await {
            Ok(()) => {},
            Err(e) => println!("Error: {}", e),
        }
        std::thread::sleep(std::time::Duration::from_secs(120));
    }
}

async fn check_and_update_ip() -> Result<(), Box<dyn std::error::Error>> {
    print!("\nGetting current IP address");
    let cur_ip = api::get_current_ip().await?;
    println!(" - {}\n", cur_ip);
    let record_ips: CloudFlareResult = api::get_record_ip(RECORDS, ZONE, &AUTH_KEY).await?;
    for i in 0..RECORDS.len() {
        if cur_ip != record_ips.result[i].content {
            print!("Updating record {} from {} to {}", record_ips.result[i].name, record_ips.result[i].content, cur_ip);
            match api::update_record(&record_ips.result[i], &cur_ip, AUTH_KEY).await {
                Ok(()) => println!(" - Record updated"),
                Err(e) => println!(" - Error: {}", e),
            }
        } else {
            println!("Record {} is up to date", record_ips.result[i].name);
        }
    }
    Ok(())
}


use config_loader::Config;

use crate::api::DNSRecordResult;

mod api;
mod config_loader;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// The main function of the program
#[tokio::main]
async fn main() {
    println!("Cloudflare IP updater v{}", VERSION);
    print!("Loading config... ");
    let config =
        config_loader::load_config(handle_args().as_str()).expect("\nConfigLoadFailException");
    println!("Loaded!");
    loop {
        match check_and_update_ip(&config).await {
            Ok(()) => {}
            Err(e) => println!("\nError: {}", e),
        }
        std::thread::sleep(std::time::Duration::from_secs(config.update_threshold));
    }
}

/// Checks the zones and their flagged records for IP address changes and updates them.
///
///  # Arguments
/// * `config` - The configuration to use
///
/// # Returns
/// * `Ok(())` - If the IP addresses were updated successfully
/// * `Err(e)` - If the IP addresses could not be updated
async fn check_and_update_ip(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting current IP addresses...");
    let cur_ip = api::get_current_ip().await?;
    for k in 0..config.keys.len() {
        println!("Updating zones for key {}", config.keys[k].auth_key);
        for z in 0..config.keys[k].zones.len() {
            println!(
                "Updating records for zone {}",
                config.keys[k].zones[z].zone_id
            );
            let a_record_ips: Vec<DNSRecordResult> = api::get_record_ip(
                &config.keys[k].zones[z].a_records,
                &config.keys[k].zones[z].zone_id,
                &config.keys[k].auth_key,
                "A",
            )
            .await?;
            let aaaa_record_ips: Vec<DNSRecordResult> = api::get_record_ip(
                &config.keys[k].zones[z].aaaa_records,
                &config.keys[k].zones[z].zone_id,
                &config.keys[k].auth_key,
                "AAAA",
            )
            .await?;
            match cur_ip.ipv4.clone() {
                Some(ipv4) => {
                    println!("\nCurrent IPv4 address: {}", ipv4);
                    for i in 0..config.keys[k].zones[z].a_records.len() {
                        if !a_record_ips.get(i).is_none() && !a_record_ips[i].locked {
                            if ipv4 != a_record_ips[i].content {
                                print!(
                                    "Updating record {} from {} to {}",
                                    a_record_ips[i].name, a_record_ips[i].content, ipv4
                                );
                                match api::update_record(
                                    &a_record_ips[i],
                                    &ipv4,
                                    &config.keys[k].auth_key,
                                    "A",
                                )
                                .await
                                {
                                    Ok(()) => println!(" - Record updated"),
                                    Err(e) => println!(" - Error: {}", e),
                                }
                            } else {
                                println!("Record {} is up to date", a_record_ips[i].name);
                            }
                        }
                    }
                }
                None => println!("No IPv4 address found, skipping A records"),
            }
            match cur_ip.ipv6.clone() {
                Some(ipv6) => {
                    println!("\nCurrent IPv6 address: {}", ipv6);
                    for i in 0..config.keys[k].zones[z].aaaa_records.len() {
                        if !aaaa_record_ips.get(i).is_none() && !aaaa_record_ips[i].locked {
                            if ipv6 != aaaa_record_ips[i].content {
                                print!(
                                    "Updating record {} from {} to {}",
                                    aaaa_record_ips[i].name, aaaa_record_ips[i].content, ipv6
                                );
                                match api::update_record(
                                    &aaaa_record_ips[i],
                                    &ipv6,
                                    &config.keys[k].auth_key,
                                    "AAAA",
                                )
                                .await
                                {
                                    Ok(()) => println!(" - Record updated"),
                                    Err(e) => println!(" - Error: {}", e),
                                }
                            } else {
                                println!("Record {} is up to date", aaaa_record_ips[i].name);
                            }
                        }
                    }
                }
                None => println!("No IPv6 address found, skipping AAAA records"),
            }
            println!("Done updating zone")
        }
        println!("Done updating keys zones")
    }
    Ok(())
}

/// Handles the input arguments.
/// Currently only custom config parameter is supported
///
/// # Returns
/// * `String` - Path to the custom config file
///
/// # Examples
///
/// ```
/// use cloudflare_dns_updater::handle_args;
/// use cloudflare_dns_updater::config_loader::load_config;
///
/// let config_path = handle_args();
/// let config = load_config(config_path.as_str()).expect("Failed to load config!");
/// ```
///
/// # Errors
/// * `std::env::VarError` - If the environment variable could not be read or parsed
/// * `std::env::VarError` - If the environment variable is not set
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

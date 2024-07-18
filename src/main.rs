mod config;
mod resolver;
use chrono::Local;
use config::Config;
use futures_util::TryStreamExt;
use reqwest::Client;
use std::fs;
use std::io::Write;
use std::net::Ipv4Addr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    let mut results = Vec::new();

    for dns in &config.dns {
        if let Ok(ip) = dns.parse::<Ipv4Addr>() {
            let size = download(&config.file_url, ip, &config.timeout)
                .await
                .unwrap_or_else(|_| {
                    println!("failed to resolve with {} dns", dns);
                    0
                });
            results.push((ip.to_string(), size));
        }
    }

    results.sort_by_key(|&(_, size)| std::cmp::Reverse(size));

    if let Some((best_dns, size)) = results.first() {
        if *size > 0 {
            println!("*********************");
            println!("Best DNS server is {}", best_dns);
            println!("*********************");
        } else {
            println!("*********************");
            println!("Network is not reachable");
            println!("*********************");
        }
    }
    write_result(results);
    Ok(())
}

async fn download(
    url: &str,
    ip: Ipv4Addr,
    timeout: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("timeout is :{}", timeout);
    let timeout_duration =
        Duration::from_secs(timeout.trim_end_matches('s').parse::<u64>().unwrap()); // Timeout duration in seconds

    let client = Client::builder()
        .dns_resolver(Arc::new(resolver::CustomDnsResolver::new(
            std::net::IpAddr::V4(ip),
        )))
        .timeout(timeout_duration)
        .build()?;
    println!("send request");
    let response = client.get(url).send().await?;
    println!("end request");
    let mut stream = response.bytes_stream();
    let mut downloaded_bytes = 0;

    let result = tokio::time::timeout(timeout_duration, async {
        while let Some(chunk) = stream.try_next().await? {
            downloaded_bytes += chunk.len();
        }
        Ok::<(), reqwest::Error>(())
    })
    .await;

    match result {
        Ok(_) => println!("Download complete!"),
        Err(_) => println!("Timeout reached!"),
    }

    println!("Downloaded bytes: {}", downloaded_bytes);

    Ok(downloaded_bytes)
}

fn write_result(results: Vec<(String, usize)>) {
    // Create the "output" directory if it doesn't exist
    let output_dir = Path::new("output");
    fs::create_dir_all(&output_dir).expect("unable to create output directory");

    // Format the file name with a timestamp, replace ':' with '-'
    let file_name = format!(
        "best403unlocker_output-{}",
        Local::now().format("%Y-%m-%d_%H-%M-%S")
    );

    let file_path = output_dir.join(file_name);

    // Create and open the file in the "output" directory
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    // Write the results to the file
    for (ip, bytes) in results {
        writeln!(file, "{},     {}", ip, bytes).expect("unable to write the results");
    }
}
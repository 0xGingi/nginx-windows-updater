use reqwest;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::time::Duration;
use schedule_recv::periodic;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://nginx.org/en/CHANGES";
    let client = reqwest::Client::new();

    let re = Regex::new(r"Changes with nginx (\d+\.\d+\.\d+)\s*(\d+\s\w+\s\d+)").unwrap();

    let tick = periodic(Duration::from_secs(3600));

    // Check version on launch
    check_version(&client, &re, &url).await?;

    loop {
        match tick.recv() {
            Ok(_) => {
                // Check version every hour
                check_version(&client, &re, &url).await?;
            },
            Err(_) => break,
        }
    }

    Ok(())
}

async fn check_version(client: &reqwest::Client, re: &Regex, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = client.get(url).send().await?;
    let content = resp.text().await?;

    if let Some(cap) = re.captures(&content) {
        let version = &cap[1];
        let date = &cap[2];

        println!("Latest version: {}, released on {}", version, date);

        let last_version = match fs::read_to_string("latest_version.txt") {
            Ok(content) => content,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    fs::write("latest_version.txt", "")?;
                    String::new()
                } else {
                    return Err(Box::new(std::io::Error::new(err.kind(), format!("Failed to read latest_version.txt: {}", err))));
                }
            }
        };
    
        if version != &last_version {
            fs::write("latest_version.txt", version)?;
            println!("New version detected: {}", version);
        }

        let output = std::process::Command::new("C:\\nginx\\nginx.exe")
        .arg("-V")
        .output()
        .expect("Failed to execute command");
    
        let local_version_output = String::from_utf8_lossy(&output.stderr);
        let local_version_re = Regex::new(r"nginx/(\d+\.\d+\.\d+)").unwrap();
    
        let local_version = local_version_output
        .lines()
        .filter_map(|line| local_version_re.captures(line))
        .next();
    
        match local_version {
            Some(local_cap) => {
                let local_version = &local_cap[1];
                println!("Local version: {}", local_version);
    
                if local_version != version {
                    println!("Local version is different from the latest version");


                    // Stop the Nginx service
                    std::process::Command::new("powershell")
                        .arg("-Command")
                        .arg("Stop-Service -Name 'Nginx'")
                        .status()
                        .expect("Failed to stop Nginx service");
                
            
                    // Download the new version
                    let download_url = format!("https://nginx.org/download/nginx-{}.zip", version);
                    let response = client.get(&download_url).send().await.map_err(|err| format!("Failed to download file from {}: {}", download_url, err))?;
            
                    if response.status().is_success() {
                        let bytes = response.bytes().await?;
                        let reader = std::io::Cursor::new(bytes);
            
                        // Extract nginx.exe from the zip file
                        let mut archive = zip::ZipArchive::new(reader)?;
                        let file_name = format!("nginx-{}/nginx.exe", version);
                        let mut file = archive.by_name(&file_name)?;
                        let mut out = Vec::new();
                        std::io::copy(&mut file, &mut out)?;
            
                        // Replace the existing nginx.exe with the new one
                        let path = std::path::Path::new("C:\\nginx\\nginx.exe");
                        let mut file = std::fs::File::create(&path)?;
                        file.write_all(&out)?;

                        // Start the Nginx service
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        std::process::Command::new("powershell")
                            .arg("-Command")
                            .arg("Start-Process powershell -Verb runAs -ArgumentList 'Start-Service -Name Nginx'")
                            .status()
                            .expect("Failed to start Nginx service");                    
            
                        println!("Updated to version {}", version);
                    } else {
                        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download the new version")));
                    }
                } else {
                    println!("Local version is up to date");
                }
            },
            None => {
                println!("Could not determine local version");
            }
        }
    }

    Ok(())
}
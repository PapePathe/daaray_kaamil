use std::env;
use std::fs::File;
use std::io::copy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    // Get the URL from the command-line arguments
    let url = &args[1];

    // Parse the file name from the URL
    let file_name = url.split('/').last().unwrap_or("downloaded_file");
    println!("Downloading {} to {}", url, file_name);

    // Send a GET request to the URL
    let response = reqwest::blocking::get(url)?;

    // Check for a successful response
    if !response.status().is_success() {
        eprintln!("Failed to download file: HTTP {}", response.status());
        std::process::exit(1);
    }
    let final_file_name = String::from("tmp/") + file_name;

    // Create a file to save the downloaded content
    let mut file = File::create(final_file_name)?;

    // Copy the content from the response into the file
    let content = response.bytes()?;
    copy(&mut content.as_ref(), &mut file)?;

    println!("File downloaded successfully as {}", file_name);

    Ok(())
}

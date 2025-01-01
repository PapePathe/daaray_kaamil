use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;
use Box;

#[derive(Debug, Deserialize)]
struct XasidaLink {
    href: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required parameter is provided
    if args.len() > 2 {
        eprintln!("Usage: {} <parameter>", args[0]);
        std::process::exit(1);
    }

    // Extract the parameter
    let parameter = &args[1];
    println!("You passed: {}", parameter);

    // Open the JSON file
    let file = File::open(parameter)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON file into the Config struct
    let links: Vec<XasidaLink> = serde_json::from_reader(reader)?;
    // Print the parsed links
    for (index, link) in links.iter().enumerate() {
        println!("Link {}: {:?}", index + 1, link);
    }

    Ok(())
}

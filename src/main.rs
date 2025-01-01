use crate::config::Config;
use crate::xasida_link::XasidaLink;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;
use Box;

pub mod config;
pub mod xasida_link;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("ERR: xasida {}", err);
        process::exit(1);
    });

    // Open the JSON file
    let file = File::open(config.source_path)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON file into the Config struct
    let links: Vec<XasidaLink> = serde_json::from_reader(reader)?;
    // Print the parsed links
    for (index, link) in links.iter().enumerate() {
        println!("Link {}: {:?}", index + 1, link);
    }

    Ok(())
}

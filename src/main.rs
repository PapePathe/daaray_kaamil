use crate::config::Config;
use crate::xasida_link::XasidaLink;
use std::env;
use std::fs::File;
use std::io::BufReader;

pub mod config;
pub mod xasida_link;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = initialize_config(&args)?;
    let links = read_links(&config.source_path)?;

    process_links(&links);
    Ok(())
}

fn initialize_config(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {
    Config::new(args).map_err(|err| {
        let message = format!("ERR: xasida {}", err);
        eprintln!("{}", message);
        Box::<dyn std::error::Error>::from(message)
    })
}

fn read_links(source_path: &str) -> Result<Vec<XasidaLink>, Box<dyn std::error::Error>> {
    let file = File::open(source_path).map_err(|err| {
        let message = format!("ERR: Failed to open file {}: {}", source_path, err);
        eprintln!("{}", message);
        Box::<dyn std::error::Error>::from(message)
    })?;
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).map_err(|err| {
        let message = format!("ERR: Failed to parse JSON: {}", err);
        eprintln!("{}", message);
        Box::<dyn std::error::Error>::from(message)
    })
}

fn process_links(links: &[XasidaLink]) {
    links.iter().enumerate().for_each(|(index, link)| {
        println!("Link {}: {:?}", index + 1, link);
    });
}

fn download_pdf(l: XasidaLink) {}

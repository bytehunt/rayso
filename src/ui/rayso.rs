use std::fs;
use rbase64;
use clap::Parser;
use crate::ui::args::Cli;
use open;

pub fn ray() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let ray_url = "https://ray.so/#code=";
    let filename = &cli.filename;
    let file_contents = fs::read(filename)?;
    let base64_encoded = rbase64::encode(&file_contents);
    let joined_url = format!("{}{}", ray_url, base64_encoded);
    println!("{}", joined_url);
    let browser = &cli.browser;
    open::with(&joined_url, browser)?;

    Ok(())
}


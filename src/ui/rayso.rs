use open;
use rbase64;
use std::fs;
use clap::Parser;
use crate::ui::args::Cli;

pub fn generate_url(
    theme: &str,
    background: bool,
    padding: i32,
    darkmode: bool,
    base64_encoded: &str,
    filename: &str,
) -> String {
    let ray_url = "https://ray.so/#code=";
    format!(
        "{}{}&darkMode={}&theme={}&title={}&background={}&padding={}",
        ray_url, base64_encoded, darkmode, theme, filename, background, padding
    )
}

pub fn ray() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let filename = &cli.filename;
    let browser = &cli.open;

    let file_contents = fs::read(filename)?;
    let base64_encoded = rbase64::encode(&file_contents);

    let joined_url = generate_url(
        &cli.theme,
        cli.background,
        cli.padding,
        cli.darkmode,
        &base64_encoded,
        filename,
    );

    println!("{}", joined_url);
    open::with(&joined_url, browser)?;

    Ok(())
}


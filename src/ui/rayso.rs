use crate::ui::args::Cli;
use clap::Parser;
use rbase64;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

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

    let joined_url_str = joined_url.to_string();

    // Use xclip command to copy to clipboard
    let xclip_process = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .spawn();

    if let Ok(mut child) = xclip_process {
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(joined_url_str.as_bytes())?;
        }
    }

    open::with(&joined_url, browser)?;
    //println!("{}", joined_url_str);

    Ok(())
}

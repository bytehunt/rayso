use crate::ui::args::Cli;
use clap::Parser;
use rbase64;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn generate_url(
    theme: &str,
    background: bool,
    padding: usize,
    darkmode: bool,
    base64_encoded: &str,
    filename: &str,
) -> String {
    const RAY_URL: &str = "https://ray.so/#code=";
    format!(
        "{}{}&darkMode={}&theme={}&title={}&background={}&padding={}",
        RAY_URL, base64_encoded, darkmode, theme, filename, background, padding
    )
}

pub fn ray() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();  // Parse command-line arguments
    let filename = &cli.filename;

    let file_contents = fs::read(filename)?;  // Read file contents
    let base64_encoded = rbase64::encode(&file_contents);  // Encode file contents to base64

    let joined_url = generate_url(
        &cli.theme,
        cli.background,
        cli.padding,
        cli.darkmode,
        &base64_encoded,
        filename,
    );  // Generate URL using provided parameters

    let mut child = Command::new("xclip")  // Spawn xclip process
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .spawn()?;  // Handle potential errors

    if let Some(stdin) = &mut child.stdin {
        stdin.write_all(joined_url.as_bytes())?;  // Write URL to xclip process stdin
    } else {
        return Err("Failed to get stdin for xclip process".into());
    }

    if cli.open {
        open::that(&joined_url)?;  // Open URL in default browser
    }

    Ok(())  // Return success
}


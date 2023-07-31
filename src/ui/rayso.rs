use crate::ui::args::Cli;
use clap::Parser;
use rbase64;
use std::process::Stdio;
use tokio::fs as async_fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command as AsyncCommand;

const RAY_URL: &str = "https://ray.so/#code=";

pub fn generate_url(
    theme: &str,
    background: bool,
    padding: usize,
    darkmode: bool,
    base64_encoded: &str,
    filename: &str,
) -> String {
    format!(
        "{}{}&darkMode={}&theme={}&title={}&background={}&padding={}",
        RAY_URL, base64_encoded, darkmode, theme, filename, background, padding
    )
}

pub async fn ray() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse(); // Parse command-line arguments
    let file_contents = async_fs::read(&cli.filename).await?; // Read file contents asynchronously
    let base64_encoded = rbase64::encode(&file_contents); // Encode file contents to base64

    let joined_url = generate_url(
        &cli.theme,
        cli.background,
        cli.padding,
        cli.darkmode,
        &base64_encoded,
        &cli.filename,
    ); // Generate URL using provided parameters

    let mut child = AsyncCommand::new("xclip") // Spawn xclip process asynchronously
        .arg("-sel")
        .arg("c")
        .stdin(Stdio::piped())
        .spawn()?; // Handle potential errors

    match child.stdin {
        Some(ref mut stdin) => {
            stdin.write_all(joined_url.as_bytes()).await?; // Write URL to xclip process stdin asynchronously
        }
        None => {
            return Err("Failed to get stdin for xclip process".into());
        }
    }

    if cli.open {
        open::that(&joined_url)?; // Open URL in default browser
    }

    Ok(()) // Return success
}

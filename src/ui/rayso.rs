use crate::ui::args::{Cli, Padding, Theme};
use clap::Parser;
use rbase64;
use std::process::Stdio;
use tokio::fs as async_fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command as AsyncCommand;

pub async fn ray() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let file_contents = async_fs::read(&cli.filename).await?;
    let base64_encoded = rbase64::encode(&file_contents);

    let padding_value = match cli.padding {
        Padding::_16 => 16,
        Padding::_32 => 32,
        Padding::_64 => 64,
        Padding::_128 => 128,
    };

    let theme_value = match cli.theme {
        Theme::_Breeze => "Breeze",
        Theme::_Candy => "Candy",
        Theme::_Crimson => "Crimson",
        Theme::_Falcon => "Falcon",
        Theme::_Meadow => "Meadow",
        Theme::_Midnight => "Midnight",
        Theme::_Raindrop => "Raindrop",
        Theme::_Sunset => "Sunset",
    };

    let joined_url = crate::ui::url_generator::generate_url(
        theme_value,
        cli.background,
        padding_value,
        cli.darkmode,
        &base64_encoded,
        &cli.filename,
    );

    let mut child = AsyncCommand::new("wl-copy") // Spawn xclip process asynchronously
        .stdin(Stdio::piped())
        .spawn()?; // Handle potential errors

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(joined_url.as_bytes()).await?; // Write URL to xclip process stdin asynchronously
    } else {
        return Err("Failed to get stdin for xclip process".into());
    }

    if cli.open {
        open::that(&joined_url)?; // Open URL in default browser
    }

    Ok(()) // Return success
}

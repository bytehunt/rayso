use crate::ui::args::{Cli, Padding, Theme};
use clap::Parser;
use rbase64;
use tokio::fs as async_fs;

pub async fn clipboard(url: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
        match session_type.as_str() {
            "x11" => crate::ui::clip::x11_clip(url).await,
            "wayland" => crate::ui::clip::wayland_clip(url).await,
            _ => Err("Unsupported session type".into()),
        }
    } else {
        Err("XDG_SESSION_TYPE environment variable not set".into())
    }
}

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

    let _ = clipboard(joined_url.as_bytes()).await;

    if cli.open {
        open::that(&joined_url)?; // Open URL in default browser
    }

    Ok(()) // Return success
}

use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command as AsyncCommand;

pub async fn write_to_clipboard(
    command: &str,
    input: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = AsyncCommand::new(command).stdin(Stdio::piped()).spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input).await?;
    } else {
        return Err("Failed to get stdin for the process".into());
    }

    child.wait().await?;
    Ok(())
}

pub async fn x11_clip(joined_url: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    write_to_clipboard("xclip", joined_url).await
}

pub async fn wayland_clip(joined_url: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    write_to_clipboard("wl-copy", joined_url).await
}

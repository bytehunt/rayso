use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "✨Upload code to ray.so from terminal"
)]
pub struct Cli {
    #[arg(short, long)]
    /// specify the filename to upload
    pub filename: String,

    #[arg(short, long, default_value = "brave")]
    /// specify the browser to open the URL
    pub open: String,

    #[arg(short, long, default_value = "candy")]
    /// specify the theme of the image
    ///
    /// Available options: Breeze, candy, Crimson, Falcon, Meadow, Midnight, Raindrop, Sunset
    pub theme: String,

    #[arg(short, long, default_value = "false")]
    /// specify if the background should be enabled
    pub background: bool,

    #[arg(short, long, default_value = "32")]
    /// specify the padding of the image
    ///
    /// Available options: 16, 32, 64, 128
    pub padding: i32,

    #[arg(short, long, default_value = "false")]
    /// specify if dark mode should be toggled
    pub darkmode: bool,
}

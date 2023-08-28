use clap::{Parser, ValueEnum};

const ABOUT: &str = "Rayo: create beautiful code snippets on ray.so";

#[derive(Parser)]
#[command(
    author,
    version,
    about = ABOUT,
)]
pub struct Cli {
    #[arg(short, long)]
    /// specify the filename to upload
    pub filename: String,

    #[arg(short, long, default_value = "false")]
    /// Specify whether to open in the default browser or not.
    pub open: bool,

    #[arg(value_enum, default_value = "candy")]
    /// specify the theme of the image
    pub theme: Theme,

    #[arg(short, long, default_value = "false")]
    /// specify if the background should be enabled
    pub background: bool,

    #[arg(value_enum, default_value = "16")]
    /// specify the padding of the image
    pub padding: Padding,

    #[arg(short, long, default_value = "false")]
    /// specify if dark mode should be toggled
    pub darkmode: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(non_camel_case_types)]
pub enum Padding {
    /// Provides a compact and minimalistic padding around the screenshot.
    _16,

    /// Offers a moderate amount of padding, allowing content to breathe.
    _32,

    /// Provides a generous padding, creating a comfortable whitespace.
    _64,

    /// Offers a spacious padding for a pronounced separation from other content.
    _128,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(non_camel_case_types)]
pub enum Theme {
    /// A light and refreshing theme reminiscent of a gentle breeze.
    _Breeze,

    /// A sweet and vibrant theme that adds a touch of playfulness.
    _Candy,

    /// A rich and deep theme that exudes elegance and passion.
    _Crimson,

    /// A sleek and dynamic theme that brings a sense of speed and motion.
    _Falcon,

    /// A calm and soothing theme inspired by the tranquility of meadows.
    _Meadow,

    /// A mysterious and enigmatic theme that evokes the night's beauty.
    _Midnight,

    /// A refreshing and revitalizing theme like the first drops of rain.
    _Raindrop,

    /// A warm and captivating theme that captures the essence of sunsets.
    _Sunset,
}

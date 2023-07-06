use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = "None")]
pub struct Cli {
    #[arg(short, long)]
    /// filename
    pub filename: String,

    #[arg(short, long, default_value = "/usr/bin/brave")]
    /// Browser to open the url
    pub browser: String,
}

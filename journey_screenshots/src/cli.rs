use std::path::PathBuf;

use clap::{ArgAction, Parser};

#[derive(Parser, Debug, Clone)]
#[command(name = "journey_screenshots", version, about = "Capture screenshots across a user journey using headless Chromium (Puppeteer-like)")]
pub struct Cli {
    /// The starting URL of the journey
    pub url: Option<String>,

    /// Output directory for screenshots
    #[arg(short = 'o', long = "out", default_value = "screenshots")] 
    pub out_dir: PathBuf,

    /// Max pages to traverse before stopping (safety guard)
    #[arg(short = 'm', long = "max", default_value_t = 100)]
    pub max_pages: usize,

    /// Delay in milliseconds after navigation before taking screenshot
    #[arg(short = 'd', long = "delay-ms", default_value_t = 500)]
    pub delay_ms: u64,

    /// Run with visible browser instead of headless
    #[arg(long, action = ArgAction::SetTrue)]
    pub headed: bool,
}



use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Confirm, Input, Select};
use tokio::time::sleep;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

mod actions;
mod browser;
mod cli;

use actions::{click_continue, query_h1_text, sanitize_filename, take_screenshot};
use browser::launch_browser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let mut cli = Cli::parse();
    let mut start_url = match cli.url.take() {
        Some(u) => u,
        None => Input::<String>::new()
            .with_prompt("Enter the starting URL")
            .interact_text()?,
    };

    // Ensure output directory exists
    std::fs::create_dir_all(&cli.out_dir).context("creating output directory")?;

    let browser = launch_browser(cli.headed).await?;
    let page = browser.new_page("about:blank").await?;

    let mut page_index: usize = 1;
    loop {
        info!("navigating to: {}", start_url);
        page.goto(start_url.as_str()).await?;
        sleep(Duration::from_millis(cli.delay_ms)).await;

        // Wait for network idle-ish: small grace
        let _ = page.wait_for_navigation().await;
        sleep(Duration::from_millis(cli.delay_ms)).await;

        // Grab the H1 text and current URL
        let current_url: String = page
            .url()
            .await
            .ok()
            .and_then(|u| u)
            .unwrap_or_else(|| start_url.clone());
        let h1_text = query_h1_text(&page).await.unwrap_or_else(|_| "".to_string());

        // Take screenshot
        let file = cli
            .out_dir
            .join(format!("{:03}_{}.png", page_index, sanitize_filename(&h1_text)));
        take_screenshot(&page, &file).await?;
        info!("saved screenshot: {}", file.display());

        // Try to click Continue or Save and continue
        match click_continue(&page).await {
            Ok(true) => {
                page_index += 1;
                // Wait for navigation after click
                let _ = page.wait_for_navigation().await;
                sleep(Duration::from_millis(cli.delay_ms)).await;
                // update URL to whatever page reports now
                start_url = page
                    .url()
                    .await
                    .ok()
                    .and_then(|u| u)
                    .unwrap_or(start_url);
                if page_index > cli.max_pages {
                    info!("Reached max pages limit: {}", cli.max_pages);
                    break;
                }
                continue;
            }
            Ok(false) => {
                // No continue found: report and let user decide
                println!("=== STOP: No Continue button found ===");
                println!("URL: {current_url}");
                println!("H1: {h1_text}");
                let choices = &["Provide a new URL and continue", "Exit program"];
                let idx = Select::new()
                    .with_prompt("Choose next action")
                    .items(choices)
                    .default(0)
                    .interact()?;
                if idx == 0 {
                    start_url = Input::<String>::new()
                        .with_prompt("Enter new URL to continue from")
                        .interact_text()?;
                    page_index += 1;
                    continue;
                } else {
                    break;
                }
            }
            Err(err) => {
                error!("error while trying continue: {err}");
                println!("=== ERROR during continue action ===");
                println!("URL: {current_url}");
                println!("H1: {h1_text}");
                let exit = Confirm::new()
                    .with_prompt("Encountered error. Exit now?")
                    .default(true)
                    .interact()?;
                if exit { break; }
                start_url = Input::<String>::new()
                    .with_prompt("Enter new URL to continue from")
                    .interact_text()?;
            }
        }
    }

    Ok(())
}

use anyhow::{anyhow, Result};
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use tracing::error;

pub async fn launch_browser(headed: bool) -> Result<Browser> {
    let mut builder = BrowserConfig::builder();
    if !headed {
        builder = builder.arg("--headless=new");
    }
    builder = builder
        .no_sandbox()
        .arg("--disable-gpu")
        .arg("--start-maximized");
    let config = builder.build().map_err(|e| anyhow!(e))?;

    let (browser, mut handler) = Browser::launch(config).await?;
    tokio::spawn(async move {
        while let Some(res) = handler.next().await {
            if let Err(err) = res {
                error!("browser handler error: {err}");
                break;
            }
        }
    });

    Ok(browser)
}



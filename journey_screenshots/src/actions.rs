use std::path::PathBuf;

use anyhow::{Context, Result};

pub async fn take_screenshot(page: &chromiumoxide::Page, path: &PathBuf) -> Result<()> {
    let bytes = page
        .screenshot(chromiumoxide::page::ScreenshotParams::default())
        .await?;
    std::fs::write(path, &bytes).with_context(|| format!("writing screenshot to {}", path.display()))?;
    Ok(())
}

pub async fn query_h1_text(page: &chromiumoxide::Page) -> Result<String> {
    let handle = page
        .evaluate("() => { const h = document.querySelector('h1'); return h ? h.textContent || '' : ''; }")
        .await?;
    let val_opt: Option<serde_json::Value> = handle.into_value()?;
    let s: String = val_opt
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();
    Ok(s)
}

pub async fn click_continue(page: &chromiumoxide::Page) -> Result<bool> {
    let script = r#"() => {
        function findByText(texts) {
            const elements = Array.from(document.querySelectorAll('button, a, input[type=\"submit\"], input[type=\"button\"], [role=\"button\"]'));
            for (const el of elements) {
                const label = (el.textContent || el.value || '').trim().toLowerCase();
                for (const t of texts) {
                    if (label === t || label.includes(t)) return el;
                }
            }
            return null;
        }
        const target = findByText(["continue", "save and continue"]);
        if (target) { target.scrollIntoView({behavior: 'instant', block: 'center'}); target.click(); return true; }
        const govuk = document.querySelector('.govuk-button');
        if (govuk) { govuk.scrollIntoView({behavior: 'instant', block: 'center'}); govuk.click(); return true; }
        return false;
    }"#;
    let res = page.evaluate(script).await?;
    let val_opt: Option<serde_json::Value> = res.into_value()?;
    let clicked = val_opt.and_then(|v| v.as_bool()).unwrap_or(false);
    Ok(clicked)
}

pub fn sanitize_filename(input: &str) -> String {
    let trimmed = input.trim();
    let fallback = if trimmed.is_empty() { "page" } else { trimmed };
    fallback
        .chars()
        .map(|c| match c { '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_', _ => c })
        .collect()
}



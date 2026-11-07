// Extracted from the original main.rs without changing the implementation.
use serde_json::Value;
use crate::parser::escape;
use crate::renderer::{markdown_to_html, preserve_raw_html_block};

#[derive(Debug, Clone, Default)]
pub(crate) struct ChapterChrome {
    pub(crate) header: Option<String>,
    pub(crate) footer: Option<String>,
}

pub(crate) fn chrome_section(name: &str, config: Option<&Value>) -> Result<Option<String>, String> {
    let Some(config) = config else { return Ok(None) };
    let map = config.as_object().ok_or_else(|| format!("[preprocessor.components.{name}] must be a table"))?;
    let enabled = match map.get("enable") {
        Some(value) => value.as_bool().ok_or_else(|| format!("{name}.enable must be a boolean"))?,
        None => false,
    };
    if !enabled { return Ok(None) }
    let text = map.get("text").and_then(Value::as_str)
        .ok_or_else(|| format!("{name}.text must be a string when enabled"))?;
    let align = map.get("align").map_or("center", |v| v.as_str().unwrap_or(""));
    if !matches!(align, "left" | "center" | "right") {
        return Err(format!("{name}.align must be left, center, or right"));
    }
    let separator = match map.get("separator") {
        Some(v) => v.as_bool().ok_or_else(|| format!("{name}.separator must be a boolean"))?,
        None => true,
    };
    // Markdown is the default: links, emphasis and inline code work directly
    // in book.toml. Raw HTML is author-controlled, just like chapter Markdown.
    // Use format = "text" to display angle brackets literally.
    let format = map.get("format").map_or("markdown", |v| v.as_str().unwrap_or(""));
    let content = match format {
        "markdown" => preserve_raw_html_block(&markdown_to_html(text)),
        "html" => preserve_raw_html_block(text),
        "text" => escape(text),
        _ => return Err(format!("{name}.format must be markdown, html, or text")),
    };
    let class = if separator { " workbook-chrome--separator" } else { "" };
    let html = format!(
        "<{name} class=\"workbook-{name}{class}\" data-align=\"{align}\">{}</{name}>\n\n",
        content.trim_end_matches('\n')
    );
    Ok(Some(html))
}

pub(crate) fn read_chrome(context: &Value) -> Result<ChapterChrome, String> {
    let settings = context.pointer("/config/preprocessor/components");
    Ok(ChapterChrome {
        header: chrome_section("header", settings.and_then(|v| v.get("header")))?,
        footer: chrome_section("footer", settings.and_then(|v| v.get("footer")))?,
    })
}


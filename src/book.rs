// Extracted from the original main.rs without changing the implementation.
use serde_json::Value;
use crate::{ChapterChrome, transform};

/// Return the position immediately after any leading semantic <header> blocks.
/// mdbook-gitinfo emits a `gitinfo-header` at the beginning of the chapter;
/// our header should follow it, not displace it. We only inspect leading
/// header blocks so a <header> nested in chapter content is never moved.
fn leading_headers_end(content: &str) -> usize {
    let mut position = 0;
    loop {
        let remainder = &content[position..];
        let whitespace = remainder.len() - remainder.trim_start().len();
        let start = position + whitespace;
        let candidate = &content[start..];
        let Some(tag) = candidate.get(..7) else { break };
        if !tag.eq_ignore_ascii_case("<header") { break; }
        if !candidate.as_bytes().get(7).is_some_and(|c| c.is_ascii_whitespace() || *c == b'>') {
            break;
        }
        let Some(open_end) = candidate.find('>') else { break };
        let after_open = start + open_end + 1;
        let Some(close_start) = content[after_open..].to_ascii_lowercase().find("</header>") else { break };
        position = after_open + close_start + "</header>".len();
    }
    position
}

/// Preserve the original chapter and any earlier preprocessor's semantic
/// headers/footers. Append the components header after leading headers and
/// the components footer after the full chapter (including existing footers).
/// Independent sibling elements avoid invalid nested header/footer markup.
fn combine_chrome(content: &str, chrome: &ChapterChrome) -> String {
    let mut updated = content.to_owned();
    if let Some(header) = &chrome.header {
        if !updated.contains("class=\"workbook-header") {
            let position = leading_headers_end(&updated);
            let mut insertion = String::new();
            if position > 0 && !updated[..position].ends_with('\n') { insertion.push_str("\n\n"); }
            insertion.push_str(header);
            if position < updated.len() && !updated[position..].starts_with('\n') { insertion.push('\n'); }
            updated.insert_str(position, &insertion);
        }
    }
    if let Some(footer) = &chrome.footer {
        if !updated.contains("class=\"workbook-footer") {
            if !updated.ends_with("\n\n") { updated.push_str("\n\n"); }
            updated.push_str(footer);
        }
    }
    updated
}

pub(crate) fn visit_book_with_chrome(node: &mut Value, chrome: &ChapterChrome) -> Result<(), String> {
    if let Some(ch) = node.get_mut("Chapter") {
        let location = ch.get("path").and_then(Value::as_str)
            .or_else(|| ch.get("name").and_then(Value::as_str))
            .unwrap_or("<unknown chapter>").to_owned();
        if let Some(content) = ch.get_mut("content") {
            let src = content.as_str().ok_or("Expected chapter content string")?;
            let transformed = transform(src).map_err(|e| format!("{location}: {e}"))?;
            *content = Value::String(combine_chrome(&transformed, chrome));
        }
        if let Some(sub) = ch.get_mut("sub_items").and_then(Value::as_array_mut) {
            for item in sub { visit_book_with_chrome(item, chrome)?; }
        }
    }
    Ok(())
}

// mdBook 0.5 renamed Book::sections to Book::items.
// Preserve the input shape rather than rewriting the protocol version.
pub(crate) fn process_book_with_chrome(book: &mut Value, chrome: &ChapterChrome) -> Result<(), String> {
    let field = if book.get("items").is_some() {
        "items"
    } else if book.get("sections").is_some() {
        "sections"
    } else {
        return Err("Expected book.items (mdBook 0.5+) or book.sections (mdBook 0.4)".into());
    };
    let chapters = book.get_mut(field)
        .and_then(Value::as_array_mut)
        .ok_or_else(|| format!("Expected book.{field} array"))?;
    for item in chapters {
        visit_book_with_chrome(item, chrome)?;
    }
    Ok(())
}

// Preserve the existing helper interface for content-only regression tests.
#[cfg(test)]
pub(crate) fn process_book(book: &mut Value) -> Result<(), String> {
    process_book_with_chrome(book, &ChapterChrome::default())
}


#[cfg(test)]
pub(crate) fn combine_chrome_for_tests(content: &str, chrome: &ChapterChrome) -> String {
    combine_chrome(content, chrome)
}

// Extracted from the original main.rs without changing the implementation.
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use crate::parser::{component_icon, escape};

pub(crate) fn markdown_to_html(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let mut events = Vec::new();
    let mut fenced_info: Option<String> = None;
    let mut code_body = String::new();

    for event in Parser::new_ext(content, options) {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                fenced_info = Some(info.to_string());
                code_body.clear();
            }
            Event::Text(text) if fenced_info.is_some() => {
                code_body.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if fenced_info.is_some() => {
                let info = fenced_info.take().unwrap_or_default();
                let mut attributes = String::new();
                let mut tokens = info.split_whitespace();
                if let Some(first) = tokens.next() {
                    if !first.contains('=') {
                        attributes.push_str(&format!(" class=\"language-{}\"", escape(first)));
                    } else {
                        tokens = info.split_whitespace();
                    }
                }
                for token in tokens {
                    if token == "showLineNumbers" {
                        attributes.push_str(" data-show-line-numbers=\"true\"");
                        continue;
                    }
                    if let Some((key, value)) = token.split_once('=') {
                        let attr = match key {
                            "start" => Some("data-start"),
                            "hl" => Some("data-hl"),
                            "linenumber" | "ln" => Some("data-ln"),
                            "showLineNumbers" => Some("data-show-line-numbers"),
                            "startLineNumber" => Some("data-start-line-number"),
                            "collapse" => Some("data-collapse"),
                            "collapseStyle" => Some("data-collapse-style"),
                            "wrap" => Some("data-wrap"),
                            _ => None,
                        };
                        if let Some(attr) = attr {
                            attributes.push_str(&format!(" {attr}=\"{}\"", escape(value)));
                        }
                    }
                }
                let source = code_body.trim_end_matches(['\n', '\r']);
                let rendered = format!("<pre><code{attributes}>{}</code></pre>\n", escape(source));
                events.push(Event::Html(CowStr::Boxed(rendered.into_boxed_str())));
            }
            _ if fenced_info.is_none() => events.push(event),
            _ => {}
        }
    }

    let mut rendered = String::new();
    html::push_html(&mut rendered, events.into_iter());
    rendered
}

// mdBook parses chapter Markdown after this preprocessor runs. A blank line
// inside emitted block HTML ends CommonMark HTML block type 6, causing the
// remaining lines to be parsed as Markdown (even inside <pre><code>).
// Remove blank separator lines in generated HTML, preserving actual blank
// lines inside <pre> by emitting an HTML newline character reference.
pub(crate) fn preserve_raw_html_block(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_pre = false;
    for line in html.split_inclusive('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with("<pre") { in_pre = true; }
        if trimmed.is_empty() {
            if in_pre { out.push_str("&#10;"); }
        } else {
            out.push_str(line);
        }
        if trimmed.contains("</pre>") { in_pre = false; }
    }
    out
}

pub(crate) fn svg_icon(drawing: &str) -> String {
    format!("<span class=\"workbook-component__icon\" aria-hidden=\"true\"><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\">{drawing}</svg></span>")
}

pub(crate) fn feature_icon(name: &str) -> &'static str {
    match name {
        "bars" | "menu" => "<path d='M4 6h16M4 12h16M4 18h16'/>",
        "magnifying-glass" | "search" => component_icon("explanation"),
        "paintbrush" => "<path d='m14 6 4 4M4 20c3 0 5-2 5-5 0-1 0-2 1-3l8-8a2 2 0 0 1 3 3l-8 8c-1 1-2 1-3 1-3 0-5 2-6 4Z'/>",
        "angle-left" => "<path d='m15:wq 18-6-6 6-6'/>",
        "angle-right" => "<path d='m9 6 6 6-6 6'/>",
        "angle-left-right" => "<path d='m10 6-6 6 6 6m4-12 6 6-6 6'/>",
        "arrow-right-arrow-left" => "<path d='M8 3 4 7l4 4M4 7h16M16 21l4-4-4-4m4 4H4'/>",
        "github" => "<path d='M9 19c-4 1-4-2-6-2m12 4v-3.5a3 3 0 0 0-.8-2.3c2.7-.3 5.5-1.4 5.5-6A4.7 4.7 0 0 0 18.5 5 4.3 4.3 0 0 0 18.4 2s-1 0-3 1.5a11 11 0 0 0-6 0C7.4 2 6.4 2 6.4 2A4.3 4.3 0 0 0 6.3 5 4.7 4.7 0 0 0 5 8.9c0 4.6 2.8 5.7 5.5 6a3 3 0 0 0-.8 2.3V21'/>",
        "pencil" => "<path d='M12 20h9M4 17l-1 4 4-1L20 7a2 2 0 0 0-3-3L4 17Z'/>",
        "terminal" => component_icon("terminal"),
        "book-open" => "<path d='M12 7c-3-2-6-2-10-1v13c4-1 7-1 10 1m0-13c3-2 6-2 10-1v13c-4-1-7-1-10 1m0-13v13'/>",
        _ => component_icon("info"),
    }
}

// Decorative, self-contained Aside icons; no legacy workbook-component markup.
pub(crate) const ASIDE_ICON_NOTEPAD: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><rect x=\"5\" y=\"2\" width=\"15\" height=\"20\" rx=\"2\"/><path d=\"M9 7h7M9 11h7M9 15h5M3 6h4M3 11h4M3 16h4\"/></svg></span>";
pub(crate) const ASIDE_ICON_BOOK: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M12 7c-3-2-6-2-10-1v13c4-1 7-1 10 1m0-13c3-2 6-2 10-1v13c-4-1-7-1-10 1m0-13v13\"/></svg></span>";
pub(crate) const ASIDE_ICON_IMPORTANT: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"M12 6v8M12 18h.01\"/></svg></span>";
pub(crate) const ASIDE_ICON_NOTE: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"M12 16v-4m0-4h.01\"/></svg></span>";
pub(crate) const ASIDE_ICON_TIP: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M9 18h6m-5 4h4M8 14c-1.5-1.2-2.5-3-2.5-5a6.5 6.5 0 0 1 13 0c0 2-1 3.8-2.5 5-.8.7-1 1.6-1 2H9c0-.4-.2-1.3-1-2Z\"/></svg></span>";
pub(crate) const ASIDE_ICON_CAUTION: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"m10.3 3.9-8.2 14a2 2 0 0 0 1.7 3h16.4a2 2 0 0 0 1.7-3l-8.2-14a2 2 0 0 0-3.4 0Z\"/><path d=\"M12 9v4m0 4h.01\"/></svg></span>";
pub(crate) const ASIDE_ICON_SUCCESS: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"m8 12 3 3 5-6\"/></svg></span>";
pub(crate) const ASIDE_ICON_EXPLANATION: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M9 9a3 3 0 0 1 6 0c0 2-3 2-3 5\"/><path d=\"M12 17h.01\"/><circle cx=\"12\" cy=\"12\" r=\"10\"/></svg></span>";
pub(crate) const ASIDE_ICON_BUG: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"m8 2 1.88 1.88M14.12 3.88 16 2M9 7.13V6a3 3 0 0 1 6 0v1.13\"/><rect x=\"5\" y=\"8\" width=\"14\" height=\"13\" rx=\"7\"/><path d=\"M12 8v13M5 12H2m3 5H2m17-5h3m-3 5h3\"/></svg></span>";
pub(crate) const ASIDE_ICON_DANGER: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M12 2 3 7v10l9 5 9-5V7l-9-5Z\"/><path d=\"M12 7v6m0 4h.01\"/></svg></span>";
pub(crate) const ASIDE_ICON_GOAL: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><circle cx=\"12\" cy=\"12\" r=\"10\"/><circle cx=\"12\" cy=\"12\" r=\"6\"/><circle cx=\"12\" cy=\"12\" r=\"2\"/></svg></span>";
pub(crate) const ASIDE_ICON_L_O: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M22 10v6\"/><path d=\"M2 10 12 5l10 5-10 5z\"/><path d=\"M6 12v5c3.5 3 8.5 3 12 0v-5\"/></svg></span>";
pub(crate) const ASIDE_ICON_CHECKLIST: &str = "<span class=\"mdc-aside__icon\" aria-hidden=\"true\"><svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"m3 6 2 2 4-4\"/><path d=\"m3 13 2 2 4-4\"/><path d=\"m3 20 2 2 4-4\"/><path d=\"M13 6h8\"/><path d=\"M13 13h8\"/><path d=\"M13 20h8\"/></svg></span>";


pub(crate) fn aside_icon(kind: &str) -> &'static str {
    match kind {
        "tip" => ASIDE_ICON_TIP,
        "note" => ASIDE_ICON_NOTEPAD,
        "important" => ASIDE_ICON_IMPORTANT,
        "example" => ASIDE_ICON_BOOK,
        "success" => ASIDE_ICON_SUCCESS,
        "explanation" => ASIDE_ICON_EXPLANATION,
        "bug" => ASIDE_ICON_BUG,
        "caution" => ASIDE_ICON_CAUTION,
        "danger" => ASIDE_ICON_DANGER,
        "goal" => ASIDE_ICON_GOAL,
        "lo" => ASIDE_ICON_L_O,
        "checklist" => ASIDE_ICON_CHECKLIST,
        _ => ASIDE_ICON_NOTE,
    }
}

// Count actual outer ordered-list items, excluding nested lists and code.
// This keeps `continue` correct when steps contain sub-lists.

// Extracted from the original main.rs without changing the implementation.
use crate::parser::{attribute, component_icon, escape, Directive};
use super::{aside_icon, feature_icon, markdown_to_html, preserve_raw_html_block, set_first_ordered_list_start, svg_icon};

// Replace placeholders after Markdown has constructed lists and paragraphs.
// This allows arbitrary ordinary components to nest without invalid HTML blocks.
fn body_html(d: &Directive) -> String {
    let mut body = markdown_to_html(&d.body);
    for (marker, html) in &d.nested_blocks {
        let paragraph = format!("<p>{marker}</p>");
        body = body.replace(&paragraph, html.trim());
        // Raw HTML preceding a marker (<a>, <br>, <li>, etc.) can cause
        // pulldown-cmark to place it inside an HTML event instead of a <p>.
        // Always replace the remaining literal marker as well.
        body = body.replace(marker, html.trim());
    }
    preserve_raw_html_block(&body)
}

pub(crate) fn render(d: Directive) -> String {
    match d.kind.as_str() {
        "hero" => {
            let eyebrow = attribute(&d.attrs, "eyebrow")
                .or_else(|| attribute(&d.attrs, "module")) // Legacy alias.
                .unwrap_or_default();
            let title = attribute(&d.attrs, "heading").or_else(|| attribute(&d.attrs, "title"));
            let tags = attribute(&d.attrs, "tags").unwrap_or_default();
            let icon = if d.show_icon { svg_icon(component_icon("terminal")) } else { String::new() };
            let eyebrow_html = if eyebrow.trim().is_empty() {
                String::new()
            } else {
                format!("<div class=\"workbook-eyebrow\">{icon}{}</div>", escape(&eyebrow))
            };
            let heading = title.map(|t| format!("<h2>{}</h2>", escape(&t))).unwrap_or_default();
            let tags_html: String = tags.split(',').map(str::trim).filter(|s| !s.is_empty())
                .map(|tag| format!("<span>{}</span>", escape(tag))).collect();
            let tags_html = if tags_html.is_empty() { String::new() }
                else { format!("<div class=\"workbook-tags\">{tags_html}</div>") };

            if d.hero_columns.len() == 2 {
                let (a, b) = (d.hero_columns[0].0, d.hero_columns[1].0);
                let left = a.unwrap_or_else(|| 100 - b.unwrap_or(50));
                let right = b.unwrap_or(100 - left);
                let first = &d.hero_columns[0].1;
                let second = &d.hero_columns[1].1;
                format!("\n<div class=\"workbook-hero workbook-hero--columns\"><div class=\"workbook-hero-content workbook-hero__columns\" style=\"--hero-primary: {left}fr; --hero-secondary: {right}fr;\"><div class=\"workbook-hero__column workbook-hero__column--primary\">{eyebrow_html}{heading}<div class=\"intro\">{first}</div>{tags_html}</div><div class=\"workbook-hero__column workbook-hero__column--secondary\">{second}</div></div></div>\n")
            } else {
                let body = body_html(&d);
                format!("\n<div class=\"workbook-hero\"><div class=\"workbook-hero-content\">{eyebrow_html}{heading}<div class=\"intro\">{body}</div>{tags_html}</div></div>\n")
            }
        }
        "column" => {
            // Width is validated and used by the enclosing Hero; no arbitrary CSS.
            body_html(&d)
        }
        "feature" => {
            let drawing = feature_icon(&attribute(&d.attrs, "icon").unwrap_or_else(|| "info".into()));
            let icon = if d.show_icon { svg_icon(drawing) } else { String::new() };
            let body = body_html(&d);
            // Per-card layout and width allow mixed compositions in a single grid.
            // Omitted attributes retain the parent `features` layout behaviour.
            let layout = match attribute(&d.attrs, "layout").as_deref() {
                Some("horizontal") => " workbook-feature--horizontal",
                Some("vertical") => " workbook-feature--vertical",
                _ => "",
            };
            let width = match attribute(&d.attrs, "width").as_deref() {
                Some("full") => " workbook-feature--full",
                Some("half") => " workbook-feature--half",
                _ => "",
            };
            let height = match attribute(&d.attrs, "height").as_deref() {
                Some("full") => " workbook-feature--height-full",
                Some("half") => " workbook-feature--height-half",
                _ => "",
            };
            format!("<div class=\"workbook-feature{layout}{width}{height}\">{icon}<strong>{}</strong><div class=\"workbook-feature__body\">{body}</div></div>", escape(&d.title))
        }
        "steps" => {
            let start = d.steps_start.unwrap_or(1);
            let body = body_html(&d);
            let body = set_first_ordered_list_start(&body, start);
            format!("\n<section class=\"workbook-steps\" aria-label=\"{}\" style=\"--step-counter-start: {}\">{body}</section>\n",
                escape(&d.title), start - 1)
        }
        "image" => {
            // src is required; only relative paths and http(s) links are allowed.
            let src = attribute(&d.attrs, "src").unwrap_or_default();
            let safe = !src.is_empty() && !src.chars().any(char::is_control)
                && !src.contains('"') && !src.contains('<') && !src.contains('>')
                && !src.trim_start().to_ascii_lowercase().starts_with("javascript:")
                && !src.trim_start().to_ascii_lowercase().starts_with("data:");
            if !safe { return "<!-- mdbook-components: invalid image src -->".into(); }
            let dimension = |name: &str| -> String {
                attribute(&d.attrs, name).and_then(|v| {
                    let valid = !v.is_empty() && v.len() <= 24
                        && v.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '%'
                            || c == 'p' || c == 'x' || c == 'r' || c == 'e' || c == 'm'
                            || c == 'v' || c == 'w' || c == 'h');
                    if !valid { return None; }
                    let value = if v.chars().all(|c| c.is_ascii_digit()) {
                        format!("{v}px")
                    } else { v };
                    Some(format!("--mdc-image-{name}: {value};"))
                }).unwrap_or_default()
            };
            let align = match attribute(&d.attrs, "align")
                .or_else(|| attribute(&d.attrs, "alignment")).as_deref() {
                Some("left") => "left", Some("right") => "right", _ => "center"
            };
            let alt = attribute(&d.attrs, "alt").unwrap_or_else(|| d.title.clone());
            let caption = if d.body.trim().is_empty() { String::new() }
                else { format!("<figcaption>{}</figcaption>", body_html(&d)) };
            format!("\n<figure class=\"mdc-image mdc-image--{align}\" style=\"{}{}\"><img src=\"{}\" alt=\"{}\" loading=\"lazy\">{caption}</figure>\n",
                dimension("width"), dimension("height"), escape(&src), escape(&alt))
        }
        "filetree" => {
            let body = body_html(&d);
            // Icons are theme-aware monochrome by default. The only option
            // is icons=none, which removes the icon slots entirely.
            let icons = if attribute(&d.attrs, "icons").as_deref() == Some("none") {
                "none"
            } else {
                "default"
            };
            format!("\n<div class=\"workbook-filetree\" role=\"group\" aria-label=\"{}\" data-icon-set=\"{icons}\">{body}</div>\n", escape(&d.title))
        }
        "tab" => {
            let label = attribute(&d.attrs, "label").unwrap_or_default();
            let icon = attribute(&d.attrs, "icon").unwrap_or_default();
            let body = body_html(&d);
            format!("<section class=\"workbook-tab-panel\" data-label=\"{}\" data-icon=\"{}\">{body}</section>", escape(&label), escape(&icon))
        }
        "tabs" => {
            let sync = attribute(&d.attrs, "syncKey")
                .or_else(|| attribute(&d.attrs, "sync"))
                .unwrap_or_default();
            format!("\n<div class=\"workbook-tabs\" data-sync-key=\"{}\">{}</div>\n", escape(&sync), d.body.trim())
        }
        "features" => {
            // Presentation is opt-in; existing books retain the two-column grid.
            // Only a fixed class suffix is emitted, never an author-supplied CSS class.
            let layout = match attribute(&d.attrs, "layout").as_deref() {
                Some("horizontal") => "horizontal",
                Some("vertical") => "vertical",
                Some("columns") => "columns",
                _ => "grid",
            };
            format!("\n<div class=\"workbook-features workbook-features--{layout}\">{}</div>\n", d.body.trim())
        },
        "code" | "terminal" | "output" => {
            // Presentation belongs to the code renderer, not a legacy card.
            // Retain fenced-code attributes for highlighting and line numbers.
            let content = body_html(&d);
            format!("\n{content}\n")
        }
        _ => {
            let content = body_html(&d);
            let label = escape(&d.title);
            let kind = match d.kind.as_str() {
                "info" | "note" => "note",
                "important" => "important",
                "example" => "example",
                "tip" => "tip",
                "success" => "success",
                "explanation" => "explanation",
                "bug" => "bug",
                "warning" | "caution" => "caution",
                "error" | "danger" => "danger",
                _ => "note",
            };
            // The type controls accent colour and the optional decorative Aside icon.
            let icon = if d.show_icon { aside_icon(kind) } else { "" };
            if d.collapsible {
                let open = if d.expanded { " open" } else { "" };
                format!("\n<details class=\"mdc-aside mdc-aside--{kind}\" data-kind=\"{}\"{open}><summary class=\"mdc-aside__title\">{icon}<span class=\"mdc-aside__label\">{label}</span></summary><div class=\"mdc-aside__body\">{content}</div></details>\n", escape(&d.kind))
            } else {
                format!("\n<aside class=\"mdc-aside mdc-aside--{kind}\" data-kind=\"{}\" aria-label=\"{label}\"><p class=\"mdc-aside__title\">{icon}<span class=\"mdc-aside__label\">{label}</span></p><div class=\"mdc-aside__body\">{content}</div></aside>\n", escape(&d.kind))
            }
        }
    }
}


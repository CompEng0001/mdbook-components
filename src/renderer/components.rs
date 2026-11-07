// Extracted from the original main.rs without changing the implementation.
use crate::parser::{attribute, component_icon, escape, Directive};
use super::{aside_icon, feature_icon, markdown_to_html, preserve_raw_html_block, set_first_ordered_list_start, svg_icon};

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
                let body = preserve_raw_html_block(&markdown_to_html(&d.body));
                format!("\n<div class=\"workbook-hero\"><div class=\"workbook-hero-content\">{eyebrow_html}{heading}<div class=\"intro\">{body}</div>{tags_html}</div></div>\n")
            }
        }
        "column" => {
            // Width is validated and used by the enclosing Hero; no arbitrary CSS.
            preserve_raw_html_block(&markdown_to_html(&d.body))
        }
        "feature" => {
            let drawing = feature_icon(&attribute(&d.attrs, "icon").unwrap_or_else(|| "info".into()));
            let icon = if d.show_icon { svg_icon(drawing) } else { String::new() };
            let body = preserve_raw_html_block(&markdown_to_html(&d.body));
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
            let mut body = markdown_to_html(&d.body);
            // Nested components are represented by paragraph placeholders while
            // Markdown parses the ordered list. Replace them *afterwards* so
            // their block HTML cannot terminate or escape its parent <li>.
            for (marker, html) in &d.nested_blocks {
                let paragraph = format!("<p>{marker}</p>");
                if !body.contains(&paragraph) {
                    // A malformed list is safer to reject than to silently
                    // place a nested component outside the intended step.
                    return format!("<!-- mdbook-components: unresolved step block {} -->", escape(marker));
                }
                body = body.replace(&paragraph, html.trim());
            }
            let body = preserve_raw_html_block(&body);
            let body = set_first_ordered_list_start(&body, start);
            format!("\n<section class=\"workbook-steps\" aria-label=\"{}\" style=\"--step-counter-start: {}\">{body}</section>\n",
                escape(&d.title), start - 1)
        }
        "filetree" => {
            let body = preserve_raw_html_block(&markdown_to_html(&d.body));
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
            let body = preserve_raw_html_block(&markdown_to_html(&d.body));
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
                _ => "grid",
            };
            format!("\n<div class=\"workbook-features workbook-features--{layout}\">{}</div>\n", d.body.trim())
        },
        "code" | "terminal" | "output" => {
            // Presentation belongs to the code renderer, not a legacy card.
            // Retain fenced-code attributes for highlighting and line numbers.
            let content = preserve_raw_html_block(&markdown_to_html(&d.body));
            format!("\n{content}\n")
        }
        _ => {
            let content = preserve_raw_html_block(&markdown_to_html(&d.body));
            let label = escape(&d.title);
            let kind = match d.kind.as_str() {
                "info" | "note" => "note",
                "tip" => "tip",
                "success" => "success",
                "explanation" => "explanation",
                "bug" => "bug",
                "warning" | "caution" => "caution",
                "error" | "important" | "danger" => "danger",
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


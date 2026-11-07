// Extracted from the original main.rs without changing the implementation.


pub(crate) const KINDS: &[&str] = &["terminal", "output", "error", "warning", "important", "tip", "note", "goal", "example", "exercise", "info", "success", "code", "caution", "explanation", "bug", "danger", "hero", "column", "features", "feature", "steps", "filetree", "tabs", "tab"];

#[derive(Debug)]
pub(crate) struct Directive {
    pub(crate) kind: String,
    pub(crate) title: String,
    pub(crate) collapsible: bool,
    pub(crate) expanded: bool,
    pub(crate) show_icon: bool,
    pub(crate) side: Option<String>,
    pub(crate) body: String,
    pub(crate) start_line: usize,
    pub(crate) attrs: String,
    pub(crate) tab_labels: Vec<String>,
    pub(crate) steps_start: Option<u64>,
    pub(crate) indent: usize,
    pub(crate) nested_blocks: Vec<(String, String)>,
    pub(crate) hero_columns: Vec<(Option<u8>, String)>,
}

pub(crate) fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        .replace('"', "&quot;").replace('\'', "&#39;")
}

// Read key=value pairs, including quoted values containing spaces. A key must
// begin at a token boundary; e.g. `subtitle=` cannot match `title=`.
pub(crate) fn attribute(attrs: &str, key: &str) -> Option<String> {
    let mut input = attrs.trim_start();
    while !input.is_empty() {
        let end = input.find(|c: char| c.is_whitespace() || c == '=').unwrap_or(input.len());
        let name = &input[..end];
        input = input[end..].trim_start();
        if !input.starts_with('=') {
            input = input.trim_start_matches(|c: char| !c.is_whitespace()).trim_start();
            continue;
        }
        input = input[1..].trim_start();
        let (value, rest) = if let Some(q) = input.chars().next().filter(|c| *c == '\"' || *c == '\'') {
            let after_quote = &input[q.len_utf8()..];
            let close = after_quote.find(q)?;
            (&after_quote[..close], &after_quote[close + q.len_utf8()..])
        } else {
            let end = input.find(char::is_whitespace).unwrap_or(input.len());
            (&input[..end], &input[end..])
        };
        if name == key { return Some(value.to_owned()); }
        input = rest.trim_start();
    }
    None
}

// Inline, dependency-free line icons. Stroke follows the selected mdBook theme.
pub(crate) fn component_icon(kind: &str) -> &'static str {
    let drawing = match kind {
        "note" | "info" => "<circle cx='12' cy='12' r='10'/><path d='M12 16v-4m0-4h.01'/>",
        "explanation" => "<circle cx='10.8' cy='10.8' r='7.3'/><path d='m16.2 16.2 5 5'/>",
        "tip" => "<path d='M9 18h6m-5 4h4M8 14c-1.5-1.2-2.5-3-2.5-5a6.5 6.5 0 0 1 13 0c0 2-1 3.8-2.5 5-.8.7-1 1.6-1 2H9c0-.4-.2-1.3-1-2Z'/>",
        "important" => "<path d='M12 2 3 13h8l-1 9 11-12h-8l1-8Z'/>",
        "warning" | "caution" => "<path d='m10.3 3.9-8.2 14a2 2 0 0 0 1.7 3h16.4a2 2 0 0 0 1.7-3l-8.2-14a2 2 0 0 0-3.4 0Z'/><path d='M12 9v4m0 4h.01'/>",
        "error" | "danger" => "<circle cx='12' cy='12' r='10'/><path d='m15 9-6 6m0-6 6 6'/>",
        "goal" => "<circle cx='12' cy='12' r='9'/><circle cx='12' cy='12' r='5'/><circle cx='12' cy='12' r='1'/>",
        "exercise" => "<path d='M8 4H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-3'/><rect x='8' y='2' width='8' height='4' rx='1'/><path d='m8 14 2 2 5-5'/>",
        "success" => "<circle cx='12' cy='12' r='10'/><path d='m8 12 3 3 5-6'/>",
        "terminal" => "<rect x='2' y='3' width='20' height='18' rx='2'/><path d='m6 9 3 3-3 3m6 0h6'/>",
        "output" => "<rect x='3' y='3' width='18' height='18' rx='2'/><path d='M7 8h10M7 12h10M7 16h6'/>",
        "code" => "<path d='m8 8-4 4 4 4m8-8 4 4-4 4m-3-12-2 16'/>",
        "steps" => "<path d='M4 6h2m4 0h10M4 12h2m4 0h10M4 18h2m4 0h10'/>",
        "filetree" => "<path d='M3 7h6l2 2h10v11H3z'/>",
        "example" => "<path d='M4 4h16v16H4zM8 8h8m-8 4h8m-8 4h4'/>",
        _ => "<circle cx='12' cy='12' r='10'/>",
    };
    // svg assembled from fixed static fragments, not user-controlled markup.
    // Render SVG itself in render(), where the chosen drawing is inserted.
    drawing
}

pub(crate) fn opening(line: &str) -> Option<Directive> {
    let s = line.trim();
    let spec = s.strip_prefix("::: ")?;
    let token = spec.split_whitespace().next()?;
    let kind = token.split('.').next()?;
    if !KINDS.contains(&kind) { return None; }
    let rest = &spec[token.len()..];
    let title = attribute(rest, "title")
        .unwrap_or_else(|| kind[..1].to_uppercase() + &kind[1..]);
    let side = if token.contains(".side.right") { Some("right".to_owned()) }
        else if token.contains(".side.left") { Some("left".to_owned()) }
        else { attribute(rest, "side").filter(|x| x == "left" || x == "right") };
    let collapsible = attribute(rest, "collapsible")
        .map(|s| s == "true").unwrap_or(false);
    let expanded = attribute(rest, "open").map(|s| s == "true").unwrap_or(false);
    let show_icon = attribute(rest, "icon").map(|s| s != "false").unwrap_or(true);
    Some(Directive { kind: kind.into(), title, collapsible, expanded, show_icon, side,
        body: String::new(), start_line: 0, attrs: rest.to_owned(), tab_labels: Vec::new(), steps_start: None, indent: 0, nested_blocks: Vec::new(), hero_columns: Vec::new() })
}

// Preserve metadata needed by highlightjs-line-numbers.js while letting pulldown-cmark
// render all non-code Markdown normally. Code content is HTML-escaped, not trimmed.

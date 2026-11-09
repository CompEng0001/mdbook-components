// Extracted from the original main.rs without changing the implementation.
use crate::renderer::{count_steps, render, steps_start};

mod directive;
pub(crate) use self::directive::*;

// Remove only the indentation used to introduce a component inside a step.
// This restores fenced-code recognition without changing its actual contents.
pub(crate) fn strip_directive_indent(line: &str, indent: usize) -> &str {
    let bytes = line.as_bytes();
    let mut count = 0;
    while count < indent && count < bytes.len() && bytes[count] == b' ' {
        count += 1;
    }
    &line[count..]
}

// A directive may be nested several layers below a Steps list item, for
// example: steps -> tabs -> tab -> code.  In that case the four-space
// Markdown-list indentation must not turn a fenced block into an indented
// literal block.  Keep the Steps body itself unchanged: its indentation is
// what makes the list structure possible.
pub(crate) fn nested_inside_steps(stack: &[Directive]) -> bool {
    stack.last().is_some_and(|d| d.kind != "steps")
        && stack.iter().any(|d| d.kind == "steps")
}

pub(crate) fn fence_marker(line: &str) -> Option<(char, usize)> {
    let s = line.trim_start();
    if line.len() - s.len() > 3 { return None; }
    let c = s.chars().next()?;
    if c != '`' && c != '~' { return None; }
    let length = s.chars().take_while(|x| *x == c).count();
    (length >= 3).then_some((c, length))
}

// Stack-based parser: structural children (features, tabs, hero columns) are
// restricted; ordinary components can nest within each other. Named closing
// directives give an unambiguous mismatch location.
pub(crate) fn transform(source: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut stack: Vec<Directive> = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    // Only the preceding Steps group within this chapter is used by `continue`.
    let mut last_steps_end: Option<u64> = None;

    for (n, line) in source.split_inclusive('\n').enumerate() {
        let trimmed = line.trim();
        if let Some((c, size)) = fence {
            let child_of_steps = nested_inside_steps(&stack);
            if let Some(d) = stack.last_mut() {
                if child_of_steps {
                    d.body.push_str(strip_directive_indent(line, d.indent));
                } else {
                    d.body.push_str(line);
                }
            } else {
                out.push_str(line);
            }
            let fence_line = if nested_inside_steps(&stack) {
                strip_directive_indent(line, stack.last().unwrap().indent)
            } else { line };
            if let Some((candidate, count)) = fence_marker(fence_line) {
                if candidate == c && count >= size && trimmed.chars().all(|x| x == c) {
                    fence = None;
                }
            }
            continue;
        }

        if let Some(top) = stack.last() {
            let named_close = trimmed.strip_prefix("::: /");
            if trimmed == ":::" || named_close.is_some() {
                if let Some(close_kind) = named_close {
                    if close_kind.trim() != top.kind {
                        return Err(format!("Mismatched closing directive at line {}: ::: /{}; expected ::: /{} for {} opened at line {} (nesting depth {})", n + 1, close_kind.trim(), top.kind, top.kind, top.start_line, stack.len()));
                    }
                }
                let mut completed = stack.pop().expect("component stack not empty");
                let kind = completed.kind.clone();
                if kind == "steps" {
                    let start = steps_start(&completed, last_steps_end)?;
                    let count = count_steps(&completed.body);
                    if count == 0 {
                        return Err(format!("steps at line {} requires an ordered Markdown list", completed.start_line));
                    }
                    last_steps_end = Some(start.checked_add(count as u64 - 1).ok_or_else(||
                        format!("steps numbering overflows at line {}", completed.start_line))?);
                    completed.steps_start = Some(start);
                }
                if kind == "hero" && !completed.hero_columns.is_empty() {
                    if completed.hero_columns.len() != 2 {
                        return Err(format!("hero at line {} requires exactly two ::: column children", completed.start_line));
                    }
                    if !completed.body.trim().is_empty() {
                        return Err(format!("hero at line {} cannot mix direct body text and ::: column children; move the text into a column", completed.start_line));
                    }
                    let a = completed.hero_columns[0].0;
                    let b = completed.hero_columns[1].0;
                    if a.zip(b).is_some_and(|(a, b)| u16::from(a) + u16::from(b) != 100) {
                        return Err(format!("hero at line {} column widths must sum to 100", completed.start_line));
                    }
                }
                if kind == "tabs" && completed.tab_labels.is_empty() {
                    return Err(format!("tabs at line {} must contain at least one tab", completed.start_line));
                }
                let label = if kind == "tab" { attribute(&completed.attrs, "label") } else { None };
                let column_width = if kind == "column" {
                    match attribute(&completed.attrs, "width") {
                        Some(value) => Some(value.parse::<u8>().ok()
                            .filter(|v| (1..=99).contains(v))
                            .ok_or_else(|| format!("column at line {} requires width=1..99 (percentage)", completed.start_line))?),
                        None => None,
                    }
                } else { None };
                let completed_indent = completed.indent;
                let html = render(completed);
                if let Some(parent) = stack.last_mut() {
                    if parent.kind == "hero" && kind == "column" {
                        parent.hero_columns.push((column_width, html));
                    } else if parent.kind == "features" && kind == "feature" {
                        parent.body.push_str(&html);
                    } else if parent.kind == "tabs" && kind == "tab" {
                        let label = label.expect("validated tab label");
                        if parent.tab_labels.contains(&label) {
                            return Err(format!("Duplicate tab label '{}' in tabs opened at line {}", label, parent.start_line));
                        }
                        parent.tab_labels.push(label);
                        parent.body.push_str(&html);
                    } else {
                        // A marker must be parsed as a Markdown paragraph so
                        // body_html() can replace it with the nested component.
                        // When a directive is inside a Steps list, its source
                        // indentation is stripped from non-Steps children.
                        // Retaining that original indentation here turns the
                        // marker into an indented code block (<pre><code>),
                        // leaking the literal MDCNESTEDBLOCK marker to users.
                        // Only Steps itself needs the four-space indentation
                        // to keep the placeholder inside its ordered-list item.
                        let marker = format!("MDCNESTEDBLOCK{}END", parent.nested_blocks.len());
                        let marker_indent = if parent.kind == "steps" {
                            completed_indent
                        } else {
                            0
                        };
                        parent.body.push_str(&format!("\n{}{}\n\n", " ".repeat(marker_indent), marker));
                        parent.nested_blocks.push((marker, html));
                    }
                } else {
                    out.push_str(&html);
                }
                continue;
            }
        }

        if trimmed.starts_with("::: /") {
            return Err(format!("Unexpected closing directive '{}' at line {}: no matching open component (nesting depth {})", trimmed, n + 1, stack.len()));
        }
        if trimmed == ":::" && stack.is_empty() {
            return Err(format!("Unexpected ::: closing directive at line {}: no component is open (nesting depth 0)", n + 1));
        }

        if let Some(mut directive) = opening(line) {
            let permitted = match stack.last() {
                None => directive.kind != "feature" && directive.kind != "tab" && directive.kind != "column",
                Some(parent) if parent.kind == "features" => directive.kind == "feature",
                Some(parent) if parent.kind == "hero" => directive.kind == "column",
                Some(parent) if parent.kind == "tabs" => directive.kind == "tab",
                Some(parent) => {
                    // Structural children remain constrained to their parents.
                    !["feature", "tab", "column"]
                        .contains(&directive.kind.as_str())
                        && parent.kind != "image" && !(parent.kind == "steps" && directive.kind == "steps")
                },
            };
            if !permitted {
                let parent = stack.last();
                let context = parent.map(|d| format!("{} opened at line {}", d.kind, d.start_line))
                    .unwrap_or_else(|| "chapter root".to_owned());
                return Err(format!("Invalid {} component at line {} inside {} (nesting depth {}). Structural children: feature inside features; tab inside tabs; column inside hero", directive.kind, n + 1, context, stack.len()));
            }
            if directive.kind == "image" && attribute(&directive.attrs, "src").is_none() {
                return Err(format!("image opened at line {} requires src=", n + 1));
            }
            if directive.kind == "tab" && attribute(&directive.attrs, "label").is_none_or(|s| s.trim().is_empty()) {
                return Err(format!("tab at line {} requires a non-empty label= attribute", n + 1));
            }
            directive.start_line = n + 1;
            directive.indent = line.len() - line.trim_start_matches(' ').len();
            stack.push(directive);
            continue;
        }

        let fence_line = if nested_inside_steps(&stack) {
            strip_directive_indent(line, stack.last().unwrap().indent)
        } else { line };
        if let Some((c, size)) = fence_marker(fence_line) {
            fence = Some((c, size));
        }
        let child_of_steps = nested_inside_steps(&stack);
        if let Some(parent) = stack.last_mut() {
            if (parent.kind == "features" || parent.kind == "tabs") && !trimmed.is_empty() && fence.is_none() {
                return Err(format!("Unexpected text inside {} at line {}: only ::: {} blocks are allowed", parent.kind, n + 1, if parent.kind == "features" { "feature" } else { "tab" }));
            }
            if parent.kind == "hero" && !trimmed.is_empty() && !parent.hero_columns.is_empty() {
                return Err(format!("Unexpected text inside hero at line {}: place content inside a ::: column", n + 1));
            }
            if child_of_steps {
                parent.body.push_str(strip_directive_indent(line, parent.indent));
            } else {
                parent.body.push_str(line);
            }
        } else {
            out.push_str(line);
        }
        if n > 2_000_000 { return Err("Chapter exceeds component parser safety limit".into()); }
    }
    if let Some(d) = stack.last() {
        let chain = stack.iter().map(|item| format!("{} (line {})", item.kind, item.start_line))
            .collect::<Vec<_>>().join(" > ");
        return Err(format!("Unclosed {} component opened at line {}; reached end of input at line {} (nesting depth {}). Open blocks: {}. Expected ::: or ::: /{}", d.kind, d.start_line, source.lines().count() + 1, stack.len(), chain, d.kind));
    }
    Ok(out)
}


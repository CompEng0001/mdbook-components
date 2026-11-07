// Extracted from the original main.rs without changing the implementation.
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use crate::parser::{attribute, Directive};

pub(crate) fn count_steps(markdown: &str) -> usize {
    let mut depth = 0usize;
    let mut target_depth = None;
    let mut count = 0usize;
    let mut finished = false;
    for event in Parser::new(markdown) {
        match event {
            Event::Start(Tag::List(start)) => {
                depth += 1;
                if !finished && target_depth.is_none() && start.is_some() {
                    target_depth = Some(depth);
                }
            }
            Event::Start(Tag::Item) if target_depth == Some(depth) => count += 1,
            Event::End(TagEnd::List(_)) => {
                if target_depth == Some(depth) {
                    target_depth = None;
                    finished = true;
                }
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    count
}

pub(crate) fn steps_start(d: &Directive, last_steps_end: Option<u64>) -> Result<u64, String> {
    if let Some(value) = attribute(&d.attrs, "start") {
        let start = value.parse::<u64>().map_err(|_| format!(
            "steps at line {} has invalid start={value}; expected a positive integer", d.start_line
        ))?;
        if start == 0 || start > 1_000_000_000 {
            return Err(format!("steps at line {} requires start between 1 and 1000000000", d.start_line));
        }
        return Ok(start); // explicit start takes precedence over continue
    }
    let continues = d.attrs.split_whitespace().any(|token| token == "continue")
        || attribute(&d.attrs, "continue").is_some_and(|value| value == "true");
    if continues {
        return Ok(last_steps_end.and_then(|n| n.checked_add(1)).unwrap_or(1));
    }
    Ok(1)
}

// Pulldown-cmark can emit either <ol> or <ol start="N">.
// Normalise the first list's native start attribute to match the visual counter.
pub(crate) fn set_first_ordered_list_start(body: &str, start: u64) -> String {
    let Some(begin) = body.find("<ol") else { return body.to_owned(); };
    let Some(relative_end) = body[begin..].find('>') else { return body.to_owned(); };
    let end = begin + relative_end + 1;
    format!("{}<ol start=\"{start}\">{}", &body[..begin], &body[end..])
}


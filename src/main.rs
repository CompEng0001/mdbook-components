use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use serde_json::Value;
use std::{env, error::Error, fs, io::{self, Read}, path::PathBuf};
use toml_edit::{value, Array, DocumentMut, Item};

mod assets;
mod book;
mod cli;
mod config;
mod parser;
mod renderer;
#[cfg(test)]
mod tests;

use assets::*;
use book::*;
use cli::*;
use config::*;
use parser::*;
use renderer::*;

fn main() {
    if let Err(e) = run() {
        eprintln!("mdbook-components: {e}");
        std::process::exit(1);
    }
}

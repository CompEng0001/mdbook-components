// Extracted from the original main.rs without changing the implementation.
use serde_json::Value;
use std::{env, error::Error, io::{self, Read}};
use crate::{init, process_book_with_chrome, read_chrome};

pub(crate) fn print_usage() {
    println!("mdbook-components {}\n\n\
Usage:\n\
  mdbook-components [--help | -h]\n\
  mdbook-components [--version | -V]\n\
  mdbook-components init [--book-dir PATH] [--css-dir PATH] [--js-dir PATH] [--force]\n\
  mdbook-components supports RENDERER\n\n\
When invoked without arguments, reads the mdBook preprocessor JSON payload from stdin.",
        env!("CARGO_PKG_VERSION"));
}

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [flag] if flag == "--help" || flag == "-h" => {
            print_usage();
            return Ok(());
        }
        [flag] if flag == "--version" || flag == "-V" => {
            println!("mdbook-components {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        [command, rest @ ..] if command == "init" => return init(rest),
        [command, renderer] if command == "supports" => {
            std::process::exit(if renderer == "html" { 0 } else { 1 });
        }
        [] => {} // Invoked by mdBook; read the JSON protocol payload.
        _ => return Err("Unknown arguments. Run 'mdbook-components --help' for usage.".into()),
    }
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut payload: Value = serde_json::from_str(&input)?;
    let entries = payload.as_array_mut().ok_or("Expected mdBook [context,book] input")?;
    if entries.len() != 2 { return Err("Expected mdBook [context,book] input".into()); }
    let chrome = read_chrome(&entries[0]).map_err(|e| format!("Invalid header/footer configuration: {e}"))?;
    let book = &mut entries[1];
    process_book_with_chrome(book, &chrome).map_err(|e| format!("Component conversion failed: {e}"))?;
    println!("{}", serde_json::to_string(book)?);
    Ok(())
}


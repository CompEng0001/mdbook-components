
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

fn main() {
    if let Err(e) = run() {
        eprintln!("mdbook-components: {e}");
        std::process::exit(1);
    }
}

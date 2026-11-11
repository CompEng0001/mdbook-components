<div align="center">
  <h1 align="center"><b>mdbook-components</b></h1>
</div>

<p align="center">
  <a href="https://crates.io/crates/mdbook-components">
    <img src="https://img.shields.io/crates/v/mdbook-components?style=for-the-badge" alt="Crates.io version" />
  </a>
  <a href="https://crates.io/crates/mdbook-components">
    <img src="https://img.shields.io/crates/d/mdbook-components?style=for-the-badge" alt="Downloads" />
  </a>
  <a href="https://docs.rs/mdbook-components">
    <img src="https://img.shields.io/docsrs/mdbook-components?style=for-the-badge" alt="Docs.rs" />
  </a>
  <a href="https://github.com/CompEng0001/mdbook-components/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/CompEng0001/mdbook-components/release.yml?&style=for-the-badge&label=CI" alt="CI status" />
  </a>
  <img src="https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust&style=for-the-badge" alt="Built with Rust" />
</p>

**Reusable, theme-aware Markdown components for [mdBook](https://rust-lang.github.io/mdBook/).**

A Rust mdBook preprocessor  based off of [starlight](https://starlight.astro.build/) providing `Asides`, `Steps`, `Tabs`, `File Trees`, `Feature grids`, introductory `hero` banners, `code` presentation, and theme-aware Markdown table styling. Technology icons `@latest` Devicon  icon font; generic icons use small inline SVG shapes.


## Documentation

Every component example includes its rendered appearance and authoring Markdown (using **Rendered** and **Markdown** tabs where practical):

- See [mdbook-components - example](https://CompEng0001.github.io/mdbook-components)

## Installation

Build and install into the included sample book:

### From source:

```sh
git clone git@github.com:CompEng0001/mdbook-components.git
cargo fmt --check
cargo test
cargo build --release
```
### From cargo:

```
cargo install mdbook-components
```

### Initialise theme:

To initialise the theme files or update them the following can be used:

```
mdbook-components init [--book-dir PATH] [--css-dir PATH] [--js-dir PATH][--force]
```

The following css and js files are appended to `book.toml` entries, based on the paths provided to `mdbook-components`: 

```toml
additional-css = ["theme/css/mdbook-components-devicon.css", "theme/css/mdbook-components.css"]
additional-js = ["theme/js/mdbook-components-icons.js", "theme/js/mdbook-components.js"]
```

## Licence

[GPL-2.0](LICENSE.md)


## Author

[CompEng0001](https://github.com/CompEng0001)


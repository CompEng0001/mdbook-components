# File Trees

File Trees render nested Markdown lists as collapsible project structures. Filenames are classified automatically; folders are indicated by a trailing `/`.

## Project structure with automatic icons

Icons are enabled by default. Recognised file types use monochrome Devicon glyphs, while folders and unrecognised files use generic interface icons. All icons inherit the active mdBook theme colour. Click a folder to collapse its contents.

::: tabs
::: tab label="Rendered" icon=monitor

::: filetree title="Rust workspace"
- mdbook-components/
  - Cargo.toml
  - Cargo.lock
  - LICENSE.md
  - README.md
  - src/
    - **main.rs**
    - tests.rs
    - cli.rs
    - assets.rs
    - book.rs
    - config.rs
    - parser/
      - directive.rs
      - mod.rs
    - renderer/
      - components.rs
      - markdown.rs
      - mod.rs
      - steps.rs
  - docs/
    - integration.rs
  - theme/
    - css/
      - mdbook-components.css
      - mdbook-components-devicon.css
    - js/
      - mdbook-components-icons.js
      - mdbook-components.js
  - .gitignore
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: filetree title="Rust workspace"
- mdbook-components/
  - Cargo.toml
  - Cargo.lock
  - LICENSE.md
  - README.md
  - src/
    - **main.rs**
    - tests.rs
    - cli.rs
    - assets.rs
    - book.rs
    - config.rs
    - parser/
      - directive.rs
      - mod.rs
    - renderer/
      - components.rs
      - markdown.rs
      - mod.rs
      - steps.rs
  - docs/
    - integration.rs
  - theme/
    - css/
      - mdbook-components.css
      - mdbook-components-devicon.css
    - js/
      - mdbook-components-icons.js
      - mdbook-components.js
  - .gitignore
:::
````

:::
:::

## Icon-free rendering

Use `icons=none` for a text-only tree. Folder expand/collapse controls remain available, and no unused icon spacing is reserved.

::: tabs
::: tab label="Rendered" icon=monitor

::: filetree title="Minimal listing" icons=none
- notes/
  - outline.md
  - references.txt
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: filetree title="Minimal listing" icons=none
- notes/
  - outline.md
  - references.txt
:::
````

:::
:::

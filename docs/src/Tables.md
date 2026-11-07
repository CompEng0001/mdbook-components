# Tables

Tables use **ordinary Markdown** (or HTML when semantic features such as merged cells are required). The stylesheet adds subtle headers, row separators and theme-aware contrast. Wide tables scroll horizontally without compressing their content.

## Standard Markdown table

::: tabs
::: tab label="Rendered" icon=monitor

| Language | Type | Primary use |
| --- | --- | --- |
| Rust | Compiled | Systems programming |
| Python | Interpreted | Data analysis |
| Kotlin | Compiled | Application development |

:::
::: tab label="Markdown" icon=markdown

````markdown
| Language | Type | Primary use |
| --- | --- | --- |
| Rust | Compiled | Systems programming |
| Python | Interpreted | Data analysis |
| Kotlin | Compiled | Application development |
````

:::
:::

## Column alignment

Use standard Markdown alignment markers: `:---` (left), `:---:` (centre), and `---:` (right). This continues to work because the table styles do not override alignment.

::: tabs
::: tab label="Rendered" icon=monitor

| Item | Quantity | Price |
| :--- | :---: | ---: |
| Microcontroller | 2 | £18.50 |
| Resistor | 10 | £0.05 |
| USB cable | 1 | £3.99 |

:::
::: tab label="Markdown" icon=markdown

````markdown
| Item | Quantity | Price |
| :--- | :---: | ---: |
| Microcontroller | 2 | £18.50 |
| Resistor | 10 | £0.05 |
| USB cable | 1 | £3.99 |
````

:::
:::

## Inline formatting and links

Cells can contain inline code, emphasis, and links, exactly as ordinary mdBook tables do.

::: tabs
::: tab label="Rendered" icon=monitor

| Command | Purpose | Reference |
| --- | --- | --- |
| `cargo check` | **Type-check** without a normal executable | [Cargo](https://doc.rust-lang.org/cargo/commands/cargo-check.html) |
| `cargo test` | Run *tests* | [Cargo](https://doc.rust-lang.org/cargo/commands/cargo-test.html) |

:::
::: tab label="Markdown" icon=markdown

````markdown
| Command | Purpose | Reference |
| --- | --- | --- |
| `cargo check` | **Type-check** without a normal executable | [Cargo](https://doc.rust-lang.org/cargo/commands/cargo-check.html) |
| `cargo test` | Run *tests* | [Cargo](https://doc.rust-lang.org/cargo/commands/cargo-test.html) |
````

:::
:::

## Wide responsive tables

This example deliberately contains more columns than comfortably fit on a narrow screen. Scroll horizontally on mobile or within a narrow window.

::: tabs
::: tab label="Rendered" icon=monitor

| Platform | Compiler | Package manager | Build command | Test command | Typical output | Documentation |
| --- | --- | --- | --- | --- | --- | --- |
| Rust | `rustc` | Cargo | `cargo build --release` | `cargo test --all-targets` | `target/release/` | [Rust](https://www.rust-lang.org/learn) |
| C++ | GCC / Clang | CMake | `cmake --build build` | `ctest --test-dir build` | `build/` | [CMake](https://cmake.org/documentation/) |
| Python | CPython | `pip` | `python -m build` | `python -m pytest` | `dist/` | [Python](https://docs.python.org/3/) |

:::
::: tab label="Markdown" icon=markdown

````markdown
| Platform | Compiler | Package manager | Build command | Test command | Typical output | Documentation |
| --- | --- | --- | --- | --- | --- | --- |
| Rust | `rustc` | Cargo | `cargo build --release` | `cargo test --all-targets` | `target/release/` | [Rust](https://www.rust-lang.org/learn) |
| C++ | GCC / Clang | CMake | `cmake --build build` | `ctest --test-dir build` | `build/` | [CMake](https://cmake.org/documentation/) |
| Python | CPython | `pip` | `python -m build` | `python -m pytest` | `dist/` | [Python](https://docs.python.org/3/) |
````

:::
:::

## HTML when necessary

Use HTML only for structures that Markdown does not support, such as `colspan`, `rowspan`, or a table `<caption>`. The same presentation styles apply automatically.

::: tabs
::: tab label="Rendered" icon=monitor

<table>
  <caption>Two-stage example</caption>
  <thead>
    <tr><th scope="col">Stage</th><th scope="col">Activity</th><th scope="col">Result</th></tr>
  </thead>
  <tbody>
    <tr><th scope="row" rowspan="2">Development</th><td>Compile</td><td>Build artefact</td></tr>
    <tr><td>Test</td><td>Test results</td></tr>
    <tr><th scope="row">Release</th><td colspan="2">Publish verified artefact</td></tr>
  </tbody>
</table>

:::
::: tab label="Markdown" icon=markdown

````html
<table>
  <caption>Two-stage example</caption>
  <thead>
    <tr><th scope="col">Stage</th><th scope="col">Activity</th><th scope="col">Result</th></tr>
  </thead>
  <tbody>
    <tr><th scope="row" rowspan="2">Development</th><td>Compile</td><td>Build artefact</td></tr>
    <tr><td>Test</td><td>Test results</td></tr>
    <tr><th scope="row">Release</th><td colspan="2">Publish verified artefact</td></tr>
  </tbody>
</table>
````

:::
:::

The responsive wrapper is added to tables when the page loads; it does not change authoring syntax or table semantics. Tables remain readable if JavaScript is disabled, although horizontal scrolling then depends on the browser's ordinary page overflow behaviour.

# Code presentation

Code enhancements apply directly to ordinary fenced code blocks, including those inside Tabs, Asides, and Steps. No component wrapper is required.

## Line numbers and highlighting

The highlighted row is specified relative to the source block. `startLineNumber` controls the displayed gutter number.

::: tabs
::: tab label="Rendered" icon=monitor

```rust showLineNumbers startLineNumber=20 hl=2
fn main() {
    let sample_count = 32;
    println!("{sample_count}");
}
```

:::
::: tab label="Markdown" icon=markdown

````markdown
```rust showLineNumbers startLineNumber=20 hl=2
fn main() {
    let sample_count = 32;
    println!("{sample_count}");
}
```
````

:::
:::

## Collapsible implementation details

Collapsed lines retain their original line numbering. Other supported `collapseStyle` values are `collapsible-start`, `collapsible-end`, and `collapsible-auto`.

::: tabs
::: tab label="Rendered" icon=monitor

```rust showLineNumbers collapse={1-3} collapseStyle=github
use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Measurements ready");
}
```

:::
::: tab label="Markdown" icon=markdown

````markdown
```rust showLineNumbers collapse={1-3} collapseStyle=github
use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Measurements ready");
}
```
````

:::
:::

## Plain code and metadata aliases

`ln=true` and `start=n` are shorter aliases for line-number options. Fenced code blocks use the same renderer regardless of where they appear.

::: tabs
::: tab label="Rendered" icon=monitor

```bash ln=false
printf 'Reading complete\n'
```

```bash ln=true start=7 hl=8
cargo check
cargo test
```

:::
::: tab label="Markdown" icon=markdown

````markdown
```bash ln=false
printf 'Reading complete\n'
```

```bash ln=true start=7 hl=8
cargo check
cargo test
```
````

:::
:::


```rust
let a: i32 = 100;
let b: f64 = 3.14;
let c: bool = true;
let d: char = 'R';
```

## Rust Playground and copying

mdBook’s Rust Playground hidden lines should not be counted as visible rows by the line-number enhancement. The copy control copies source text rather than displayed gutter numbers or collapse affordances. Horizontal overflow stays inside the code viewport.

## Copying source

Copy buttons preserve the original source, including newlines, indentation and
blank lines. Rebuild book assets after upgrading (`mdbook-components init --force`)
so stale JavaScript is not served by the deployed book.

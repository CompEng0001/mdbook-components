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

Collapsed lines retain their original line numbering. 


### Collapsible Style start

Use `collapseStyle=start` to keep the disclosure above the expanded code (**default**)

::: tabs
::: tab label="Rendered" icon=monitor

```rust start=1 collapse={1-3} collapseStyle=start
use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Measurements ready");
}
```

:::
::: tab label="Markdown" icon=markdown

````markdown
```rust start=1 collapse={1-3} collapseStyle=start
use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Measurements ready");
}
```
````

:::
:::

### Collapsible Style end
Use `collapseStyle=end` to keep the disclosure below the expanded code (**default**)
::: tabs
::: tab label="Rendered" icon=monitor

```rust showLineNumbers collapse={1-3} collapseStyle=end
use std::fmt;e
use std::collections::HashMap;

fn main() {
    println!("Measurements ready");
}
```

:::
::: tab label="Markdown" icon=markdown

````markdown
```rust showLineNumbers collapse={1-3} collapseStyle=end
use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Measurements ready");
}
```
````

:::
:::


### Collapsible Style default

Omitting `collapseStyle` is equivalent to `collapseStyle=start`.

::: tabs
::: tab label="Rendered" icon=monitor

```rust showLineNumbers collapse={1-3}
use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Measurements ready");
}
```

:::
::: tab label="Markdown" icon=markdown

````markdown
```rust showLineNumbers collapse={1-3}
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

## Rust Playground and copying

mdBook’s Rust Playground hidden lines should not be counted as visible rows by the line-number enhancement. The copy control copies source text rather than displayed gutter numbers or collapse affordances. Horizontal overflow stays inside the code viewport.

## Copying source

Copy buttons preserve the original source, including newlines, indentation and
blank lines. 

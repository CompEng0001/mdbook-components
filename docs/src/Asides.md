# Asides

Asides communicate context, guidance, outcomes and problems without interrupting the main narrative. Each type has its own accent and icon. All colours remain readable against mdBook's light and dark themes.

## Types and purposes

| Directive | Accent | When to use |
| --- | --- | --- |
| `note` | Blue | Additional factual context |
| `tip` | Purple | Useful practice or suggestion |
| `explanation` | Teal | Why something happens or how it works |
| `success` | Green | A completed action or expected outcome |
| `caution` | Orange | A potential issue requiring care |
| `bug` | Red | A defect, unexpected behaviour or debugging task |
| `danger` | Red | A serious risk or destructive action |

## All seven variants

::: tabs
::: tab label="Rendered" icon=monitor

::: note title="Context"
Rust variables are immutable unless explicitly declared `mut`.
:::

::: tip title="Good practice"
Run `cargo fmt --check` before opening a pull request.
:::

::: explanation title="Why does this happen?"
`cargo check` type-checks a crate without producing its usual final executable, so it is often faster during development.
:::

::: success title="Build complete"
The project compiled successfully and all tests passed.
:::

::: caution title="Overwriting files"
The shell operator `>` replaces an existing file's contents.
:::

::: bug title="Unexpected behaviour"
The program panics when the input is empty. Add a regression test and investigate the failing path.
:::

::: danger title="Security risk"
Never commit private keys or API tokens to version control.
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: note title="Context"
Rust variables are immutable unless explicitly declared `mut`.
:::

::: tip title="Good practice"
Run `cargo fmt --check` before opening a pull request.
:::

::: explanation title="Why does this happen?"
`cargo check` type-checks a crate without producing its usual final executable, so it is often faster during development.
:::

::: success title="Build complete"
The project compiled successfully and all tests passed.
:::

::: caution title="Overwriting files"
The shell operator `>` replaces an existing file's contents.
:::

::: bug title="Unexpected behaviour"
The program panics when the input is empty. Add a regression test and investigate the failing path.
:::

::: danger title="Security risk"
Never commit private keys or API tokens to version control.
:::
````

:::
:::

## Collapsible and icon-free asides

Set `collapsible=true` to render native `<details>`; `open=true` initially expands it. `icon=false` hides the decorative SVG.

::: tabs
::: tab label="Rendered" icon=monitor

::: explanation title="How does it work?" collapsible=true
The compiler checks the program's types before producing machine code.
:::

::: bug title="Known issue" icon=false
The icon is intentionally hidden in this example.
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: explanation title="How does it work?" collapsible=true
The compiler checks the program's types before producing machine code.
:::

::: bug title="Known issue" icon=false
The icon is intentionally hidden in this example.
:::
````

:::
:::

## Aliases

`info` maps to **note**, `warning` to **caution**, and `error` or `important` to **danger**. `success`, `explanation` and `bug` are distinct styles, not aliases.

# Hero

The `hero` component creates an introductory banner for a chapter, practical exercise, or documentation landing page. It uses the active mdBook theme. Authors can use a **single content area**, or place content into **two responsive columns** with adjustable proportions.

The Hero is concerned with **layout only**. QR codes are generated independently by `mdbook-qr`; no QR-specific Hero setting is required.

## Basic Hero

A Hero needs no columns. Use `eyebrow` for a short contextual label, `heading` for the principal heading and comma-separated `tags` for keywords. Its body accepts ordinary Markdown.

::: tabs
::: tab label="Rendered" icon=monitor

::: hero eyebrow="ELEE1147" heading="Programming for Engineers" tags="Rust,CLI,Build"

Build reliable software by understanding the tools, not merely memorising commands.

:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: hero eyebrow="ELEE1147" heading="Programming for Engineers" tags="Rust,CLI,Build"

Build reliable software by understanding the tools, not merely memorising commands.

:::
````

:::
:::

## Two-column Hero

Use **exactly two** `::: column` directives within a Hero to create a two-column layout. If neither column specifies a width, they share the available space equally. The eyebrow, heading and tags belong to the **first** column, followed by that column's Markdown body.

::: tabs
::: tab label="Rendered" icon=monitor

::: hero eyebrow="GETTING STARTED" heading="A reproducible Rust workflow" tags="Create,Check,Test"

::: column width=70

Organise your project with Cargo, verify each change, and document the outcome.

- Create a package with `cargo new`.
- Check it with `cargo check`.
- Execute tests with `cargo test`.

:::

::: column width=30

**Quick reference**

`cargo new` → `cargo check` → `cargo test`

For longer guidance, see the [Rust book](https://doc.rust-lang.org/book/).

:::

:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: hero eyebrow="GETTING STARTED" heading="A reproducible Rust workflow" tags="Create,Check,Test"

::: column width=70

Organise your project with Cargo, verify each change, and document the outcome.

- Create a package with `cargo new`.
- Check it with `cargo check`.
- Execute tests with `cargo test`.

:::

::: column width=30

**Quick reference**

`cargo new` → `cargo check` → `cargo test`

For longer guidance, see the [Rust book](https://doc.rust-lang.org/book/).

:::

:::
````

:::
:::

## Equal columns and inferred widths

Omit both widths for an equal split. When only one column has a `width`, the other receives the remaining percentage. Explicit widths must total 100.

::: tabs
::: tab label="Rendered" icon=monitor

::: hero eyebrow="COMPARISON" heading="Two approaches" tags="Design,Evaluation"

::: column

**Approach A**

Prefer a straightforward implementation when clarity is the primary objective.

:::

::: column

**Approach B**

Evaluate optimisations after the baseline is correct and measurable.

:::

:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: hero eyebrow="COMPARISON" heading="Two approaches" tags="Design,Evaluation"

::: column

**Approach A**

Prefer a straightforward implementation when clarity is the primary objective.

:::

::: column

**Approach B**

Evaluate optimisations after the baseline is correct and measurable.

:::

:::
````

:::
:::

## Using a QR code from mdbook-qr

`hero` can nest images, for example if your book also enables <a href="compeng0001.github.io/mdbook-qr" target="_blank">mdbook-qr</a>, place its `{{QR_CODE}}` marker in a column. The QR preprocessor replaces the marker, subject to your configured preprocessor order and marker settings.

::: tabs
::: tab label="Rendered" icon=monitor

::: hero eyebrow="...roll" heading="This isn't a rickroll..." tags="meme, fun, playful, original"

::: column width=70

... is this where it all began?

:::

::: column width=30

{{QR_CODE}}

:::

:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: hero eyebrow="...roll" heading="This isn't a rickroll..." tags="meme, fun, playful, original"

::: column width=70

... is this where it all began?

:::

::: column width=50

{{ QR_CODE}}

:::

:::
````
:::

::: tab label="book.toml" icon=toml

```
[preprocessor.qr]
enable = true
localhost-qr = true
url = "/path/to/some/location/somewhere"
qr-path = "docs/src/figures/qr.png"
width = 200
height = 200

```

:::
:::

## Attributes and behaviour

| Directive | Attribute | Effect |
|-----------|-----------|--------|
| `hero` | `eyebrow="..."` | Small label above the main heading; optional |
| `hero` | `heading="..."` | Main Hero heading; `title` is also recognised |
| `hero` | `tags="A,B,C"` | Comma-separated labels; optional |
| `hero` | `icon=false` | Hides the decorative eyebrow icon |
| `column` | `width=70` | Percentage of the two-column space; optional |

**Layout rules:**

- **No columns:** the Hero body renders normally at full width.
- **Two columns:** the first column receives the Hero heading and tags; the second receives its own Markdown body.
- **No widths:** the columns default to 50/50.
- **One width:** the other width is inferred (for example, `width=70` gives 70/30).
- **Two widths:** they must total 100, and each explicit width must be between 1 and 99.
- **Narrow screens:** columns stack into a single column.

A Hero with `::: column` children must contain **exactly two columns** and no additional text directly inside the Hero. Put all body content inside a column. A standalone `::: column` outside a Hero is invalid.

## Authoring guidance

Use a single-column Hero for a short introduction. Prefer a two-column Hero when the secondary content is genuinely complementary, such as a quick reference, diagram, link collection, or independently generated QR code. Avoid putting lengthy instructional sequences in the narrow column; use the [Steps](EXAMPLE_3.md) component below the Hero instead.

When no `::: column` children are provided, the hero content takes the full
available width. Two explicit columns retain the existing responsive grid.

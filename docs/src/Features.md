# Feature cards

Feature cards present a small collection of related capabilities, learning outcomes, or navigation choices. They use the active mdBook theme and do not require custom HTML.

Each `::: features` container holds **only** `::: feature` children. A card accepts `title` and an optional `icon`; its body supports regular Markdown such as emphasis, inline code, and links. Icons are theme-aware, monochrome SVGs.

## A three-card feature grid

The default `layout=grid` displays compact cards in two columns where space permits. The cards adjust to the available content width.

::: tabs
::: tab label="Rendered" icon=monitor

::: features
::: feature title="Write" icon=pencil
Write a small, readable Rust programme using `cargo new`.
:::
::: feature title="Build" icon=terminal
Run `cargo check` and interpret any diagnostics.
:::
::: feature title="Review" icon=github
Commit a verified change to version control.
:::
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: features
::: feature title="Write" icon=pencil
Write a small, readable Rust programme using `cargo new`.
:::
::: feature title="Build" icon=terminal
Run `cargo check` and interpret any diagnostics.
:::
::: feature title="Review" icon=github
Commit a verified change to version control.
:::
:::
````

:::
:::


## Mixed card layouts and widths

For a dashboard-like arrangement, set `layout` and `width` on **each `::: feature`**. The outer `::: features` stays a responsive two-column grid. `width=full` spans both columns; `width=half` spans one. A horizontal card defaults to full width unless you specify `width=half`.

::: tabs
::: tab label="Rendered" icon=monitor

::: features
::: feature title="Getting started" icon=book-open layout=horizontal width=full
A wide introductory card can provide a longer overview of the workflow, with its icon beside the text.
:::
::: feature title="Develop" icon=code layout=vertical width=half
Create a small Rust project using `cargo new`.
:::
::: feature title="Verify" icon=terminal layout=vertical width=half
Run `cargo check` and `cargo test` before committing.
:::
::: feature title="Next steps" icon=arrow-right-arrow-left layout=horizontal width=full
Review the results and proceed to the next exercise.
:::
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: features
::: feature title="Getting started" icon=book-open layout=horizontal width=full
A wide introductory card with its icon beside the text.
:::
::: feature title="Develop" icon=code layout=vertical width=half
Create a small Rust project using `cargo new`.
:::
::: feature title="Verify" icon=terminal layout=vertical width=half
Run `cargo check` and `cargo test`.
:::
::: feature title="Next steps" icon=arrow-right-arrow-left layout=horizontal width=full
Review results and proceed to the next exercise.
:::
:::
````

:::
:::

The two half-width cards sit side by side on wide screens and stack on narrow screens. Standard grid cards can appear alongside customised cards without specifying any attributes.

## Mixed card widths and heights

Combine `width=full|half` with `height=full|half` on individual cards. Width controls column span; height controls row span. A full-height card occupies **two grid rows**, allowing two half-height cards to sit beside it. These values describe relative grid spans, **not fixed pixel heights**.

::: tabs
::: tab label="Rendered" icon=monitor

::: features
::: feature title="Project overview" icon=book-open layout=vertical width=half height=full
A taller card for a detailed introduction.

Start with the requirements, then plan the implementation and note the expected results.
:::
::: feature title="Build" icon=code layout=horizontal width=half height=half
Compile with `cargo check`.
:::
::: feature title="Test" icon=terminal layout=horizontal width=half height=half
Run `cargo test`.
:::
::: feature title="Review" icon=github layout=horizontal width=full height=half
Summarise the outcome before proceeding.
:::
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: features
::: feature title="Project overview" icon=book-open layout=vertical width=half height=full
A taller card for a detailed introduction.

Start with the requirements, then plan the implementation and note the expected results.
:::
::: feature title="Build" icon=code layout=horizontal width=half height=half
Compile with `cargo check`.
:::
::: feature title="Test" icon=terminal layout=horizontal width=half height=half
Run `cargo test`.
:::
::: feature title="Review" icon=github layout=horizontal width=full height=half
Summarise the outcome before proceeding.
:::
:::
````

:::
:::

On narrow screens, all cards stack in document order and use their natural content height. Omit `height` to retain the previous automatic grid sizing. Content is never clipped to enforce a fixed height.

## Horizontal cards

Use `layout=horizontal` when descriptions are longer, or when you want each card to occupy a complete row. Icons appear beside the title and description rather than above them. On narrow displays, the layout remains readable without introducing horizontal scrolling.

::: tabs
::: tab label="Rendered" icon=monitor

::: features layout=horizontal
::: feature title="Develop" icon=code
Build a small working application, applying language fundamentals, readable naming, and version control throughout the process.
:::
::: feature title="Test" icon=terminal
Run automated checks and examine failures before committing your changes. Use `cargo test` to execute your Rust test suite.
:::
::: feature title="Document" icon=book-open
Explain the purpose of the implementation, its dependencies, and how another developer can reproduce the results.
:::
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: features layout=horizontal
::: feature title="Develop" icon=code
Build a small working application with readable code.
:::
::: feature title="Test" icon=terminal
Run `cargo test` and investigate failures.
:::
::: feature title="Document" icon=book-open
Explain how to reproduce the results.
:::
:::
````

:::
:::

## Vertical cards

Use `layout=vertical` for a single-column stack while retaining the familiar icon-above-heading appearance. This works well for more detailed learning objectives or a linear progression of activities.

::: tabs
::: tab label="Rendered" icon=monitor

::: features layout=vertical
::: feature title="Plan" icon=book-open
Identify requirements, establish the scope, and decide what the finished project should demonstrate.
:::
::: feature title="Implement" icon=code
Develop the application in small, testable increments.
:::
::: feature title="Evaluate" icon=terminal
Review the outcome and document limitations or improvements.
:::
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: features layout=vertical
::: feature title="Plan" icon=book-open
Identify requirements and scope.
:::
::: feature title="Implement" icon=code
Develop in small increments.
:::
::: feature title="Evaluate" icon=terminal
Review the outcome and limitations.
:::
:::
````

:::
:::

### Choosing a layout

| Attribute | Appearance | Recommended use |
| --- | --- | --- |
| `layout=grid` | Two-column grid (default) | Compact summaries and navigation |
| `layout=horizontal` | Full-width cards, icon beside text | Longer descriptions |
| `layout=vertical` | Full-width cards, icon above text | Sequential or detailed content |

For mixed layouts, omit the outer `layout` attribute and set `layout`, `width`, and optionally `height` on each card. Group-level layouts remain available for existing pages. All layouts inherit the mdBook theme and collapse gracefully on small screens. Existing feature grids require no changes.

## Markdown inside cards

A feature body may contain ordinary Markdown links, bold text and inline code. You do not need to create separate HTML cards for these cases.

::: tabs
::: tab label="Rendered" icon=monitor

::: features
::: feature title="Documentation" icon=book-open
Read the [Rust Book](https://doc.rust-lang.org/book/) to understand **ownership** and borrowing.
:::
::: feature title="Inspect" icon=search
Use `cargo clippy` to identify possible code improvements.
:::
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: features
::: feature title="Documentation" icon=book-open
Read the [Rust Book](https://doc.rust-lang.org/book/) to understand **ownership** and borrowing.
:::
::: feature title="Inspect" icon=search
Use `cargo clippy` to identify possible code improvements.
:::
:::
````

:::
:::

## Cards without icons

Set `icon=false` on an individual feature to omit its decorative icon, without changing the other cards. Use this sparingly when the text alone communicates the feature clearly.

::: tabs
::: tab label="Rendered" icon=monitor

::: features
::: feature title="Readable" icon=false
Short headings and focused descriptions are easier to scan.
:::
::: feature title="Consistent" icon=false
Use the same structure for comparable items.
:::
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: features
::: feature title="Readable" icon=false
Short headings and focused descriptions are easier to scan.
:::
::: feature title="Consistent" icon=false
Use the same structure for comparable items.
:::
:::
````

:::
:::

## Supported attributes

| Directive | Attribute | Purpose |
| --- | --- | --- |
| `features` | None required | Groups cards into a responsive layout |
| `feature` | `layout=horizontal` or `layout=vertical` | Overrides that card’s icon arrangement |
| `feature` | `width=full` or `width=half` | Spans two or one grid column |
| `feature` | `height=full` or `height=half` | Spans two or one grid row |
| `feature` | `title="..."` | Sets the visible card heading |
| `feature` | `icon=name` | Selects an available decorative icon |
| `feature` | `icon=false` | Hides the decorative icon |

Supported built-in Feature icon names include `pencil`, `terminal`, `github`, `book-open`, `search`, `paintbrush`, `menu`, `angle-left`, `angle-right`, `angle-left-right`, and `arrow-right-arrow-left`. Unknown names currently use the default information icon.

::: caution title="Nesting rule"
Only `::: feature` elements are valid **direct children** of `::: features`. Put text, links, and Markdown formatting inside each `::: feature`, not directly in the outer container.
:::

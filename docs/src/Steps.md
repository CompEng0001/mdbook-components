# Steps

Steps are numbered by the Rust preprocessor. Each group is visually independent, but numbering can resume later in the same chapter.

## A sequence separated by an explanation

The second group displays steps **3 and 4**. `continue` resumes from the last group within this chapter; intervening prose does not reset the counter.

::: tabs
::: tab label="Rendered" icon=monitor

::: steps title="Prepare the project"
1. Create a new Rust package using `cargo new sensors`.
2. Enter the directory with `cd sensors`.
:::

Run `cargo --version` to check that the toolchain is accessible.

::: steps continue title="Complete the build"
1. Run `cargo check` to detect type errors.

    ::: tabs
    ::: tab label="Rendered" icon=monitor

    ::: code
    ```rust
    int main(){

        return 0;
    }
    ```
    :::
    
    :::
    :::

2. Run `cargo test` to execute the test suite.
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: steps title="Prepare the project"
1. Create a new Rust package using `cargo new sensors`.
2. Enter the directory with `cd sensors`.
:::

Run `cargo --version` to check that the toolchain is accessible.

::: steps continue title="Complete the build"
1. Run `cargo check` to detect type errors.
2. Run `cargo test` to execute the test suite.
:::
````

:::
:::

## Explicit start and independent restart

`start=10` displays **10 and 11**. A subsequent group without `continue` starts again at **1**. Explicit `start=n` takes precedence if both attributes are supplied.

::: tabs
::: tab label="Rendered" icon=monitor

::: steps start=10 title="Numbered checkpoint"
1. Record the benchmark environment.
2. Repeat the measurement.
:::

::: steps title="New procedure"
1. Save the results.
2. Document the findings.
:::

:::
::: tab label="Markdown" icon=markdown

````markdown
::: steps start=10 title="Numbered checkpoint"
1. Record the benchmark environment.
2. Repeat the measurement.
:::

::: steps title="New procedure"
1. Save the results.
2. Document the findings.
:::
````

:::
:::

## Numbering rules

`continue=true` is equivalent to `continue`; `continue=false` resets the group. Nested Markdown lists and code blocks inside a step do not count as additional top-level steps. A new chapter always starts its own numbering state.

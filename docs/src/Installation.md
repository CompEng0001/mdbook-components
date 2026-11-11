# Installation

This chapter explains the installation process, whilst using some of the features of mdbook-components. 

## Prerequisites

::: steps title="Prepare your environment"
1. **Install mdBook.** The `mdbook` command must be available on your `PATH`.

2. **Install Rust and Cargo** if you plan to compile `mdbook-components` from source.

3. **Create or open an mdBook project** containing a `book.toml` file.
:::


## Install the preprocessor

::: steps continue

1. Choose the installation approach that suits your workflow.

    ::: tabs
    ::: tab label="From GitHub" icon=github

    Install the preprocessor from its repository:

    ```sh
    cargo install --git https://github.com/CompEng0001/mdbook-components.git
    ```

    This is suitable for book authors who want to install directly from the upstream source.

    :::
    ::: tab label="Local checkout" icon=terminal

    From a checkout of the **mdbook-components repository**:

    ```sh
    cargo install --path .
    ```

    This is useful when developing or testing changes to the preprocessor itself.

    ::: important
    Run `cargo install --path .` from the **preprocessor source directory**, not from the directory containing your book's `book.toml`.
    :::

    :::
    :::

2. Verify that the executable is accessible:

    ```sh
    mdbook-components --version
    ```

    ::: tip
    If the command is not found after a Cargo installation, ensure Cargo's binary directory (typically `~/.cargo/bin`) is included in the `PATH` used by your shell and by `mdbook build`.
    :::

:::

## Initialise your book

::: steps title="Configure and preview" continue
1. **Open your book's root directory** — the one containing `book.toml`.
2. **Initialise component assets and configuration** with `mdbook-components init`.
3. **Build the book** to run the preprocessor.
4. **Serve the generated book** and check the example below in your browser.

    ```sh
    cd path/to/my-book
    mdbook-components init
    mdbook build
    mdbook serve --open
    ```

    ::: tip
    The initialiser creates the component CSS and JavaScript directories if necessary, installs the bundled assets, adds them to the book's HTML output configuration, and enables the components preprocessor. It retains unrelated asset registrations.
    :::

    ::: tabs
    ::: tab label="Generated configuration" icon=toml

    The relevant `book.toml` configuration is equivalent to:

    ```toml
    [preprocessor.components]
    renderers = ["html"]

    [output.html]
    additional-css = [
        "theme/css/mdbook-components-devicon.css",
        "theme/css/mdbook-components.css",
    ]
    additional-js = [
        "theme/js/mdbook-components-icons.js",
        "theme/js/mdbook-components.js",
    ]
    ```

    ::: important
    These entries are shown for reference. **Do not add duplicate entries** after running `mdbook-components init`. Preserve any other existing `[output.html]` settings.
    :::

    :::
    ::: tab label="Expected files" icon=folder

    After initialisation, your book contains the component-managed assets alongside its normal content:

    ::: filetree title="Initialised mdBook project"
    - my-book/
      - book.toml
      - src/
        - SUMMARY.md
      - theme/
        - css/
          - mdbook-components.css
          - mdbook-components-devicon.css
        - js/
          - mdbook-components.js
          - mdbook-components-icons.js
    :::

    The initialiser manages these component files. Keep your own style overrides in a separate stylesheet.

    :::
    :::

:::

## Verify the installation

::: steps continue

1. Add the following to a Markdown chapter, rebuild, and inspect the resulting page:

    ::: tabs
    ::: tab label="Rendered" icon=monitor

    ::: tip title="Installation successful"
    This aside is rendered by **mdbook-components**.
    :::

    :::
    ::: tab label="Markdown" icon=markdown

    ````markdown
    ::: tip title="Installation successful"
    This aside is rendered by **mdbook-components**.
    :::
    ````

    :::
    :::

    If the aside is styled correctly, the preprocessor and component stylesheet are being applied. You can now use the other components described in the [overview](Overview.md).

:::

## Updating or reininstall

::: steps continue

1. After initialisation, your book contains the component-managed assets alongside its normal content:

    ::: filetree title="Initialised mdBook project"
    - my-book/
      - book.toml
      - src/
        - SUMMARY.md
      - theme/
        - css/
          - mdbook-components.css
          - mdbook-components-devicon.css
        - js/
          - mdbook-components.js
          - mdbook-components-icons.js
    :::

:::

## Initialising

::: steps continue

1. When upgrading the preprocessor, refresh its installed theme files:

    ```sh
    mdbook-components init --force
    mdbook build
    ```

    ::: caution
    `--force` replaces differing **component-managed assets**. Back up any edits to those generated files before running it. Place long-term custom styling in a separate stylesheet instead.
    :::

:::

### Custom asset directories

::: steps continue

1. Use `--css-dir` and `--js-dir` to specify where the component stylesheets and JavaScript files are installed. Both paths must be relative to the directory containing `book.toml`.

    ::: tabs
    ::: tab label="Default directories" icon=folder

    ```sh
    mdbook-components init
    ```

    Installs CSS into `theme/css` and JavaScript into `theme/js`.

    :::
    ::: tab label="Custom directories" icon=terminal

    ```sh
    mdbook-components init \
      --css-dir theme/styles \
      --js-dir theme/scripts
    ```

    Updates the corresponding `additional-css` and `additional-js` paths in `book.toml`.

    :::
    ::: tab label="Another book" icon=book-open

    ```sh
    mdbook-components init \
      --book-dir path/to/my-book \
      --css-dir theme/css \
      --js-dir theme/js
    ```

    The CSS and JavaScript directories remain relative to the specified book root.

    :::
    :::

    ::: important
    The command defaults to `--book-dir . --css-dir theme/css --js-dir theme/js`. Use `--force` when replacing existing component assets that differ from the bundled versions. Keep custom CSS and JavaScript in separate files.
    :::

2. To see the full CLI help:

    ```sh
    mdbook-components init --help
    ```

:::

## Troubleshooting

| Symptom | What to check |
|---|---|
| `mdbook-components: command not found` | Verify the Cargo binary directory is on `PATH`. |
| `The "components" preprocessor exited unsuccessfully` | Read the earlier error message for the affected Markdown file and line number. |
| An `Unknown component` error | Check the `:::` directive name for a spelling mistake. |
| Components render without styling | Run `mdbook-components init --force`, rebuild, and hard-refresh your browser. |
| A tab or file-tree icon is missing | Verify the icon JavaScript and Devicon stylesheet load. Devicon font assets may need network access. |
| Changes to CSS or JavaScript are not visible | Ensure updated assets were installed into the book and clear the browser cache. |

::: tip title="Next steps"
Explore the [component overview](Overview.md), then try adding Tabs, Steps, and Asides to your own chapters.
:::

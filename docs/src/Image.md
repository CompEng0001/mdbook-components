# Images

Use `::: image` to place a responsive illustration. Unlike a Markdown image,
this directive provides alignment, width, height and optional caption text.

::: tabs
::: tab label="Rendered" icon=monitor

::: image src="./figures/stars_trees.png" alt="Stars and trees" width=76% height=320 align=center
Stars and trees.
:::
:::

::: tab label="Markdown" icon=markdown
````markdown
::: image src="../figures/stars_trees.png" alt="Stars and trees" width=75% height=320 align=center
Stars and trees.
:::
````
:::
:::

- `src` (required): image path or URL. Paths are resolved relative to the chapter.
- `alt`: accessible description (recommended).
- `width`, `height`: numbers (pixels by default), or CSS dimensions such as `75%`, `320px`, `20rem`.
- `align`: `left`, `center` (default) or `right`.
- Body text is used as the image caption and may contain Markdown.

Width is responsive, never exceeding the content container. Height is applied
using `object-fit: contain` to avoid cropping.

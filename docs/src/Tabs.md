# Tabs

Tabs organise alternative instructions without duplicating pages. Choose a platform below; the two groups share `syncKey="platform-demo"`, so their selections stay in sync.

## Rendered example

::: tabs syncKey="platform-demo"
::: tab label="NixOS" icon=nixos

```bash
nix develop
```

:::
::: tab label="Ubuntu" icon=ubuntu

```bash
sudo apt install cargo rustc
```

:::
:::

The second group uses the same key and labels:

::: tabs syncKey="platform-demo"
::: tab label="NixOS" icon=terminal

```bash
nix flake show
```

:::
::: tab label="Ubuntu" icon=terminal

```bash
cargo --version
```

:::
:::

## Markdown source

````markdown
::: tabs syncKey="platform-demo"
::: tab label="NixOS" icon=nixos

```bash
nix develop
```

:::
::: tab label="Ubuntu" icon=ubuntu

```bash
sudo apt install cargo rustc
```

:::
:::

::: tabs syncKey="platform-demo"
::: tab label="NixOS" icon=terminal

```bash
nix flake show
```

:::
::: tab label="Ubuntu" icon=terminal

```bash
cargo --version
```

:::
:::
````

## Authoring and accessibility

`::: tabs` contains one or more `::: tab` blocks, each with a **unique required** `label`. `icon` is optional and accepts registered Devicon names or interface symbols. Matching `syncKey` (or `sync`) values synchronise selected labels; leave it out for independent groups. Use **Left/Right** to change tabs and **Home/End** to jump between endpoints. Without JavaScript, the panels remain readable in sequence.

Tabs can contain Markdown and other components, but this reference deliberately keeps the nested composition out of the demonstration to make both the rendered result and its source easy to inspect.

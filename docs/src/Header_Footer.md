# Global header and footer

The header and footer are **configured once in `book.toml`** and inserted automatically into every chapter. There are no `::: header` or `::: footer` directives.

## Configuration

```toml
[preprocessor.components.header]
enable = true
text = "📘 Programming for Engineers • 你好"
align = "center"
separator = true

[preprocessor.components.footer]
enable = true
text = "🦀 Rust • © 2026\nUniversity teaching material ✅"
align = "center"
separator = true
```

The gallery itself has header and footer enabled, so you can inspect the actual rendering above and below this page. Text retains emojis, international scripts, arrows and line breaks. HTML-sensitive characters such as `<`, `>` and `&` are safely escaped, not interpreted as markup.

## Options

| Setting | Value | Default |
| --- | --- | --- |
| `enable` | `true` or `false` | `false` |
| `text` | UTF-8 TOML string | Required when enabled |
| `align` | `left`, `center`, `right` | `center` |
| `separator` | `true` or `false` | `true` |

To omit either element, set its `enable` value to `false`. Header and footer are configured independently and apply to all chapters, including nested chapters. No JavaScript is needed.

### Multiline text

TOML also supports multiline strings:

```toml
[preprocessor.components.footer]
enable = true
text = """🦀 Rust Programming
© 2026 • University teaching material"""
```

Emoji appearance depends on fonts installed on the reader's device; actual Unicode characters remain unchanged in the generated HTML.

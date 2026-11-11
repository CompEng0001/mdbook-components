// Existing unit and regression tests, moved without alteration.
    use super::*;
    #[test]
    fn unicode_chrome_is_html_escaped_and_per_chapter() {
        let context = serde_json::json!({"config":{"preprocessor":{"components":{
            "header":{"enable":true,"format":"text","text":"📘 Rust & C <intro>\n日本語 🦀","align":"left","separator":true},
            "footer":{"enable":true,"text":"© 2026 → 🚀","align":"right","separator":false}
        }}}});
        let chrome = read_chrome(&context).unwrap();
        let mut book = serde_json::json!({"items":[{"Chapter":{"name":"One","content":"# Welcome\n","sub_items":[{"Chapter":{"name":"Two","content":"**Hi**","sub_items":[]}}]}}]});
        process_book_with_chrome(&mut book, &chrome).unwrap();
        for ptr in ["/items/0/Chapter/content", "/items/0/Chapter/sub_items/0/Chapter/content"] {
            let html = book.pointer(ptr).unwrap().as_str().unwrap();
            assert!(html.contains("📘 Rust &amp; C &lt;intro&gt;\n日本語 🦀"));
            assert!(html.contains("© 2026 → 🚀"));
            assert!(html.contains("<header class=\"workbook-header workbook-chrome--separator\" data-align=\"left\">"));
            assert!(html.contains("<footer class=\"workbook-footer\" data-align=\"right\">"));
            assert_eq!(html.matches("<header").count(), 1);
            assert_eq!(html.matches("<footer").count(), 1);
        }
    }

    #[test]
    fn header_footer_are_disabled_by_default_and_independently() {
        let cfg = serde_json::json!({"config":{"preprocessor":{"components":{"header":{"enable":false,"text":"Ignored"},"footer":{"enable":true,"text":"✅"}}}}});
        let chrome = read_chrome(&cfg).unwrap();
        assert!(chrome.header.is_none());
        assert!(chrome.footer.as_deref().unwrap().contains("✅"));
        assert!(read_chrome(&serde_json::json!({})).unwrap().header.is_none());
    }

    #[test]
    fn chrome_rejects_invalid_alignment_and_text_type() {
        for config in [
            serde_json::json!({"config":{"preprocessor":{"components":{"header":{"enable":true,"text":"X","align":"justify"}}}}}),
            serde_json::json!({"config":{"preprocessor":{"components":{"footer":{"enable":true,"text":42}}}}}),
        ] {
            assert!(read_chrome(&config).is_err());
        }
    }


    #[test]
    fn global_chrome_supports_markdown_links_and_utf8() {
        let section = serde_json::json!({
            "enable": true,
            "text": "📘 [Module guide](https://example.org/guide) and **Rust** 🦀"
        });
        let html = chrome_section("header", Some(&section)).unwrap().unwrap();
        assert!(html.contains("href=\"https://example.org/guide\""), "{html}");
        assert!(html.contains("<strong>Rust</strong>"), "{html}");
        assert!(html.contains("📘") && html.contains("🦀"));
    }

    #[test]
    fn global_chrome_supports_html_links_and_literal_text_mode() {
        let html_section = serde_json::json!({
            "enable": true, "format": "html",
            "text": "<a href=\"https://example.org\">Project home</a> ✅"
        });
        let html = chrome_section("footer", Some(&html_section)).unwrap().unwrap();
        assert!(html.contains("<a href=\"https://example.org\">Project home</a>"));
        assert!(html.contains("✅"));
        let raw_section = serde_json::json!({
            "enable": true, "format": "text",
            "text": "<a href=\"/\">literal</a> & 🦀"
        });
        let raw = chrome_section("footer", Some(&raw_section)).unwrap().unwrap();
        assert!(raw.contains("&lt;a href=&quot;/&quot;&gt;literal&lt;/a&gt; &amp; 🦀"));
    }

    #[test]
    fn chrome_rejects_invalid_format() {
        let config = serde_json::json!({"enable":true,"format":"rich", "text":"Test"});
        assert!(chrome_section("header",Some(&config)).is_err());
    }
    #[test]
    fn steps_render_ordered_list_and_markdown() {
        let src = "::: steps title=Build\n1. **Create** a project.\n\n   `cargo new demo`\n\n2. **Build** the project.\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("class=\"workbook-steps\""));
        assert!(html.contains("aria-label=\"Build\""));
        assert!(html.contains("<ol start=\"1\">"));
        assert_eq!(html.matches("<li>").count(), 2);
        assert!(html.contains("<strong>Create</strong>"));
        assert!(html.contains("cargo new demo"));
    }

    #[test]
    fn steps_restart_by_default() {
        let src = "::: steps\n1. First\n2. Second\n:::\n\n::: steps\n1. Again\n:::\n";
        let html = transform(src).unwrap();
        assert_eq!(html.matches("<ol start=\"1\">").count(), 2);
        assert_eq!(html.matches("--step-counter-start: 0").count(), 2);
    }

    #[test]
    fn steps_continue_after_intervening_prose() {
        let src = "::: steps\n1. First\n2. Second\n:::\n\nA paragraph.\n\n::: steps continue\n1. Third\n2. Fourth\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("<ol start=\"3\">"));
        assert!(html.contains("--step-counter-start: 2"));
    }

    #[test]
    fn steps_explicit_start_and_following_continue() {
        let src = "::: steps start=7\n1. First\n2. Second\n:::\n::: steps continue\n1. Third\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("<ol start=\"7\">"));
        assert!(html.contains("<ol start=\"9\">"));
    }

    #[test]
    fn steps_explicit_start_overrides_continue() {
        let src = "::: steps\n1. First\n:::\n::: steps continue start=12\n1. Second\n:::\n";
        assert!(transform(src).unwrap().contains("<ol start=\"12\">"));
    }

    #[test]
    fn steps_nested_lists_do_not_advance_counter() {
        let src = "::: steps\n1. First\n   - nested one\n   - nested two\n2. Second\n:::\n::: steps continue\n1. Third\n:::\n";
        assert!(transform(src).unwrap().contains("<ol start=\"3\">"));
    }

    #[test]
    fn nested_ordered_substeps_keep_parent_numbering() {
        let src = "::: steps start=2\n1. First\n2. Second\n\n   1. Nested alpha\n   2. Nested beta\n\n3. Third\n:::\n::: steps continue\n1. Fourth\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("<ol start=\"2\">"));
        assert!(html.contains("Nested alpha"));
        assert!(html.contains("Nested beta"));
        assert!(html.contains("<ol start=\"5\">"));
    }

    #[test]
    fn steps_invalid_start_and_missing_list_are_errors() {
        for invalid in ["0", "-1", "oops"] {
            let src = format!("::: steps start={invalid}\n1. First\n:::\n");
            assert!(transform(&src).is_err());
        }
        assert!(transform("::: steps continue\nNot a list\n:::\n").is_err());
    }

    #[test]
    fn steps_continue_first_group_starts_at_one() {
        let html = transform("::: steps continue\n1. First\n:::\n").unwrap();
        assert!(html.contains("<ol start=\"1\">"));
    }

    #[test]
    fn steps_continuation_is_scoped_to_each_chapter_transform() {
        let first = transform("::: steps start=20\n1. One\n:::\n").unwrap();
        let second = transform("::: steps continue\n1. One\n:::\n").unwrap();
        assert!(first.contains("<ol start=\"20\">"));
        assert!(second.contains("<ol start=\"1\">"));
    }

    #[test]
    fn steps_explicit_continue_true_and_false() {
        let src = "::: steps\n1. First\n2. Second\n:::\n::: steps continue=true\n1. Third\n:::\n::: steps continue=false\n1. Reset\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("<ol start=\"3\">"));
        assert_eq!(html.matches("<ol start=\"1\">").count(), 2);
    }

    #[test]
    fn steps_can_continue_inside_tabs() {
        let src = "::: tabs\n::: tab label=First\n::: steps start=4\n1. Alpha\n:::\n:::\n::: tab label=Second\n::: steps continue\n1. Beta\n:::\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("<ol start=\"5\">"));
    }

    #[test]
    fn filetree_renders_nested_markdown_lists() {
        let src = "::: filetree title=Project\n- demo/\n  - Cargo.toml\n  - src/\n    - **main.rs**\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("class=\"workbook-filetree\""));
        assert!(html.contains("aria-label=\"Project\""));
        assert!(html.matches("<ul>").count() >= 2);
        assert!(html.contains("<strong>main.rs</strong>"));
    }

    #[test]
    fn filetree_uses_theme_aware_icons_by_default() {
        let html = transform("::: filetree\n- src/\n:::\n").unwrap();
        assert!(html.contains("data-icon-set=\"default\""));
    }

    #[test]
    fn filetree_can_disable_icons() {
        let html = transform("::: filetree icons=none\n- Cargo.toml\n:::\n").unwrap();
        assert!(html.contains("data-icon-set=\"none\""));
    }

    #[test]
    fn filetree_legacy_and_unknown_icon_modes_fall_back_to_default() {
        for set in ["color", "lucide", "unrecognised"] {
            let input = format!("::: filetree icons={set}\n- Cargo.toml\n:::\n");
            assert!(transform(&input).unwrap().contains("data-icon-set=\"default\""));
        }
    }

    #[test]
    fn steps_preserve_fenced_code_and_directives() {
        let src = "::: steps\n1. Run this:\n\n   ```sh\n   echo '::: tip'\n   ```\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("workbook-steps"));
        assert!(html.contains("echo"));
        assert!(!html.contains("mdc-aside--tip"));
    }

    #[test]
    fn filetree_escapes_labels() {
        let html = transform("::: filetree title='A & B <C>'\n- src/\n:::\n").unwrap();
        assert!(html.contains("A &amp; B &lt;C&gt;"));
    }

    #[test]
    fn unclosed_steps_and_filetree_fail_with_line_number() {
        for name in ["steps", "filetree"] {
            let src = format!("Intro\n::: {name}\n- item\n");
            let err = transform(&src).unwrap_err();
            assert!(err.contains("line 2"));
            assert!(err.contains(name));
        }
    }

    #[test]
    fn existing_components_still_render() {
        let html = transform("::: note\n**Important**\n:::\n").unwrap();
        assert!(html.contains("mdc-aside--note"));
        assert!(html.contains("<strong>Important</strong>"));
    }

    #[test]
    fn nested_feature_icon_is_available() {
        let html = transform("::: features\n::: feature icon=angle-left-right title=Navigation\nMove between pages.\n:::\n:::\n").unwrap();
        assert!(html.contains("m10 6-6 6 6 6"));
    }
    #[test]
    fn modern_converts() {
        let s = "::: tip title=\"Helpful hint\" collapsible=true\nUse **Git**.\n:::\n";
        let out = transform(s).unwrap();
        assert!(out.contains("<details"));
        assert!(out.contains("Helpful hint"));
        assert!(out.contains("<strong>Git</strong>"));
    }
    #[test]
    fn ordinary_fenced_code_has_no_legacy_header_or_icons() {
        let html = transform("```rust ln=false hl=2\nlet x = 1;\nlet y = 2;\n```\n").unwrap();
        assert!(html.contains("<pre><code class=\"language-rust\""));
        assert!(html.contains("data-ln=\"false\""));
        assert!(html.contains("data-hl=\"2\""));
        assert!(!html.contains("workbook-component"));
        assert!(!html.contains("mdc-aside__icon"));
    }

    #[test]
    fn removed_code_wrappers_report_unknown_component() {
        for kind in ["code", "terminal", "output"] {
            let source = format!("::: {kind}\n```sh\necho hello\n```\n:::\n");
            assert!(transform(&source).is_err(), "{kind} must no longer be accepted");
        }
    }

    #[test]
    fn aside_is_semantic_and_has_no_legacy_card() {
        for (kind, expected) in [("note", "note"), ("tip", "tip"), ("warning", "caution"), ("error", "danger")] {
            let input = format!("::: {kind} title='A & B'\nExample\n:::\n");
            let html = transform(&input).unwrap();
            assert!(html.contains(&format!("mdc-aside--{expected}")));
            assert!(html.contains("<aside"));
            assert!(html.contains("A &amp; B"));
            assert!(!html.contains("workbook-component"));
            assert!(html.contains("mdc-aside__icon"));
            assert!(html.contains("aria-hidden=\"true\""));
        }
    }

    #[test]
    fn aside_collapsible_preserves_title_and_body() {
        let html = transform("::: caution title=Careful collapsible=true open=true\n**Dangerous**\n:::\n").unwrap();
        assert!(html.contains("<details class=\"mdc-aside mdc-aside--caution\""));
        assert!(html.contains(" open>"));
        assert!(html.contains("<summary class=\"mdc-aside__title\">"));
        assert!(html.contains("mdc-aside__label\">Careful</span>"));
        assert!(html.contains("<strong>Dangerous</strong>"));
    }

    #[test]
    fn aside_icons_map_to_semantic_type_and_are_decorative() {
        for (kind, expected) in [("note", "note"), ("info", "note"), ("tip", "tip"), ("success", "success"), ("warning", "caution"), ("caution", "caution"), ("error", "danger"), ("important", "important"), ("example", "example")] {
            let html = transform(&format!("::: {kind}\nHello\n:::\n")).unwrap();
            assert!(html.contains("mdc-aside__icon"), "{kind}");
            assert!(html.contains("aria-hidden=\"true\""), "{kind}");
            assert!(html.contains("mdc-aside__label"), "{kind}");
            assert!(html.contains(&format!("mdc-aside--{expected}")), "{kind}");
        }
    }

    #[test]
    fn aside_icon_false_omits_svg_but_preserves_label() {
        let html = transform("::: note title='Without icon' icon=false\nText\n:::\n").unwrap();
        assert!(!html.contains("mdc-aside__icon"));
        assert!(!html.contains("<svg"));
        assert!(html.contains("Without icon"));
    }

    #[test]
    fn collapsible_aside_keeps_icon_inside_summary() {
        let html = transform("::: tip title='More' collapsible=true\nText\n:::\n").unwrap();
        assert!(html.contains("<summary class=\"mdc-aside__title\">"));
        assert!(html.contains("mdc-aside__icon"));
    }

    #[test]
    fn plain_code_fence_unchanged() {
        let s = "```text\n::: tip\n```\n";
        // No directives are detected inside an ordinary code fence.
        assert_eq!(transform(s).unwrap(), s);
    }
    #[test]
    fn unclosed_fails() {
        assert!(transform("::: tip\nhello\n").is_err());
    }
    #[test]
    fn mdbook_05_items() {
        let mut book = serde_json::json!({"items": [
            {"Chapter": {"content": "::: tip\nHello\n:::\n", "sub_items": [
                {"Chapter": {"content": "```text\nDone\n```\n", "sub_items": []}}
            ]}}
        ]});
        process_book(&mut book).unwrap();
        assert!(book["items"][0]["Chapter"]["content"].as_str().unwrap().contains("mdc-aside--tip"));
        assert!(book["items"][0]["Chapter"]["sub_items"][0]["Chapter"]["content"].as_str().unwrap().contains("Done"));
    }
    #[test]
    fn mdbook_04_sections() {
        let mut book = serde_json::json!({"sections": [
            {"Chapter": {"content": "::: tip\nHello\n:::\n", "sub_items": []}}
        ]});
        process_book(&mut book).unwrap();
        assert!(book["sections"][0]["Chapter"]["content"].as_str().unwrap().contains("mdc-aside--tip"));
    }
    #[test]
    fn unclosed_reports_chapter_and_line() {
        let mut book = serde_json::json!({"items": [
            {"Chapter": {"path": "Documentation.md", "content": "Hello\n::: info\nSome content\n", "sub_items": []}}
        ]});
        let err = process_book(&mut book).unwrap_err();
        assert!(err.contains("Documentation.md"));
        assert!(err.contains("line 2"));
        assert!(err.contains(":::"));
    }
    #[test]
    fn nested_code_in_continuing_steps_inside_tabs() {
        let source = r#"::: tabs
::: tab label="Rendered" icon=monitor
::: steps title="Prepare the project"
1. Create a package.
2. Enter the directory.
:::
::: steps continue title="Complete the build"
1. Run `cargo check`.

    ```c
    #include <stdio.h>

    int main(){
        return 0;
    }
    ```

2. Run `cargo test`.
:::
:::
::: tab label="Markdown"
````markdown
::: steps
1. Example
:::
````
:::
:::"#;
        let rendered = transform(source).expect("Steps should accept an ordinary nested code fence");
        assert!(rendered.contains("<ol start=\"3\">"), "{rendered}");
        assert!(rendered.contains("<pre><code class=\"language-c\">"), "{rendered}");
        assert!(rendered.contains("#include &lt;stdio.h&gt;"), "{rendered}");
        assert!(rendered.contains("int main(){"), "{rendered}");
        assert!(!rendered.contains("MDCNESTEDBLOCK"), "{rendered}");
        let code_position = rendered.find("<pre><code class=\"language-c\">").unwrap();
        let list_item_start = rendered[..code_position].rfind("<li>").unwrap();
        let list_item_end = rendered[code_position..].find("</li>").unwrap() + code_position;
        assert!(list_item_start < code_position && code_position < list_item_end);
        assert!(rendered.contains("Run <code>cargo test</code>"), "{rendered}");
    }

    #[test]
    fn tabs_with_code_nested_inside_continuing_steps() {
        let source = r#"::: tabs
::: tab label="Rendered" icon=monitor
::: steps title="Prepare the project"
1. Create a package.
2. Enter the directory.
:::
::: steps continue title="Complete the build"
1. Run `cargo check`.

    ::: tabs
    ::: tab label="Rendered" icon=monitor

    ```rust
    fn main() {
        println!("Hello");
    }
    ```

    :::
    :::

2. Run `cargo test`.
:::
:::
::: tab label="Markdown" icon=markdown
````markdown
::: steps
1. Example
:::
````
:::
:::"#;
        let html = transform(source).expect("deeply nested tabs/code should render");
        assert!(html.contains("<ol start=\"3\">"), "{html}");
        assert_eq!(html.matches("class=\"workbook-tabs\"").count(), 2, "{html}");
        assert!(html.contains("<pre><code class=\"language-rust\">"), "{html}");
        assert!(html.contains("println!(\"Hello\");"), "{html}");
        assert!(!html.contains("```rust"), "{html}");
        assert!(!html.contains("MDCNESTEDBLOCK"), "{html}");
        let code_at = html.find("<pre><code class=\"language-rust\">").unwrap();
        let li_before = html[..code_at].rfind("<li>").expect("preceding list item");
        let li_end = html[code_at..].find("</li>").expect("closing list item") + code_at;
        assert!(li_before < code_at && code_at < li_end, "{html}");
    }

    #[test]
    fn deeply_nested_code_placeholder_is_not_an_indented_code_block() {
        // The fenced block is indented for its parent Steps list, but the
        // rendered code belongs to a Tab whose Markdown body is de-indented.
        let input = r#"::: steps
1. Run `cargo check`.

    ::: tabs
    ::: tab label="Rendered"

    ```rust
    fn main() { println!("Hello"); }
    ```

    :::
    :::

2. Run `cargo test`.
:::"#;
        let output = transform(input).unwrap();
        assert!(!output.contains("MDCNESTEDBLOCK"), "{output}");
        assert!(output.contains("workbook-tabs"), "{output}");
        assert!(output.contains("language-rust"), "{output}");
        assert!(output.contains("Run <code>cargo test</code>"), "{output}");
    }

    #[test]
    fn nested_aside_inside_step() {
        let source = "::: steps\n1. Check the result.\n\n    ::: caution title=\"Be careful\"\n    Do not overwrite data.\n    :::\n\n2. Save it.\n:::\n";
        let html = transform(source).unwrap();
        assert!(html.contains("mdc-aside--caution"), "{html}");
        assert!(!html.contains("MDCNESTEDBLOCK"), "{html}");
        assert_eq!(html.matches("<li>").count(), 2, "{html}");
    }

    #[test]
    fn nested_reports_location() {
        let html = transform("::: info\n::: tip\nInner\n:::\n:::\n").unwrap();
        assert!(html.contains("mdc-aside--tip"));
        assert!(html.contains("mdc-aside--note"));
    }
    #[test]
    fn blank_lines_in_code_fences_do_not_break_html_blocks() {
        let input = "```sh\ngit config --global\n\nasdasd\n```\n";
        let result = transform(input).unwrap();
        assert!(result.contains("</code></pre>"));
        assert!(result.contains("git config --global\n&#10;asdasd"));
        assert!(!result.contains("\n\n"), "output HTML must not contain blank separator lines");
    }
    #[test]
    fn blank_lines_between_markdown_paragraphs_do_not_break_html_blocks() {
        let result = transform("::: error\nFirst paragraph.\n\nSecond paragraph.\n:::\n").unwrap();
        assert!(result.contains("<p>First paragraph.</p>"));
        assert!(result.contains("<p>Second paragraph.</p>"));
        assert!(!result.contains("\n\n"));
    }
    #[test]
    fn bad_book_shape_has_actionable_error() {
        let mut book = serde_json::json!({"other": []});
        assert!(process_book(&mut book).unwrap_err().contains("book.items"));
    }

    #[test]
    fn custom_title_and_icon_render() {
        let html = transform("::: info title=\"Git configuration details\" collapsible=true\nHello\n:::\n").unwrap();
        assert!(html.contains("<details"));
        assert!(html.contains("Git configuration details"));
        assert!(!html.contains("<svg"));
        assert!(html.contains("mdc-aside"));
    }

    #[test]
    fn explanation_has_magnifying_glass_icon() {
        let html = transform("::: explanation title=\"Why this works\" collapsible=true\nDetails\n:::\n").unwrap();
        assert!(html.contains("data-kind=\"explanation\""));
        assert!(!html.contains("<svg"));
        assert!(html.contains("Why this works"));
        assert!(html.contains("<details"));
    }

    #[test]
    fn open_and_hide_icon() {
        let html = transform("::: goal title='Learning objective' collapsible=true open=true icon=false\nHello\n:::\n").unwrap();
        assert!(html.contains("<details"));
        assert!(html.contains(" open>"));
        assert!(!html.contains("workbook-component__icon"));
    }


    #[test]
    fn title_is_html_escaped() {
        let html = transform("::: note title=\"A & B <C>\"\nMessage\n:::\n").unwrap();
        assert!(html.contains("A &amp; B &lt;C&gt;"));
    }

    #[test]
    fn hero_and_tags_render_without_qr_coupling() {
        let input = "::: hero module=ELEE1119 tags=\"Linux, Single-board computers\"\n\nWelcome to **ELEE1119**.\n\n::: impl\n";
        // Unsupported closing directives must be treated as text, not markers.
        assert!(transform(input).is_err());
        let input = "::: hero module=ELEE1119 tags=\"Linux, Single-board computers\"\n\nWelcome to **ELEE1119**.\n\n:::\n";
        let html = transform(input).unwrap();
        assert!(html.contains("class=\"workbook-hero\""));
        assert!(!html.contains("{{QR_CODE}}"));
        assert!(html.contains("Single-board computers"));
        assert!(html.contains("<strong>ELEE1119</strong>"));
    }

    #[test]
    fn nested_feature_grid_renders_two_cards() {
        let input = "::: features\n\n::: feature icon=bars title=\"Chapter sidebar\"\nBrowse chapters.\n:::\n\n::: feature icon=magnifying-glass title=Search\nFind commands.\n:::\n\n:::\n";
        let html = transform(input).unwrap();
        assert!(html.contains("class=\"workbook-features workbook-features--grid\""));
        assert_eq!(html.matches("class=\"workbook-feature\"").count(), 2);
        assert!(html.contains("Chapter sidebar"));
        assert!(html.contains("Find commands"));
    }

    #[test]
    fn feature_layout_variants_are_explicit_and_backwards_compatible() {
        let cases = [
            ("", "grid"),
            (" layout=grid", "grid"),
            (" layout=horizontal", "horizontal"),
            (" layout=vertical", "vertical"),
            (" layout=unknown", "grid"),
        ];
        for (attribute, expected) in cases {
            let source = format!("::: features{attribute}\n::: feature title=Alpha icon=terminal\nBody.\n:::\n:::\n");
            let html = transform(&source).unwrap();
            assert!(html.contains(&format!("workbook-features--{expected}")), "{attribute}");
            assert!(html.contains("Alpha"));
            assert!(html.contains("Body."));
        }
    }

    #[test]
    fn mixed_features_support_per_card_layout_and_width() {
        let source = "::: features\n::: feature title=Overview icon=terminal layout=horizontal width=full\nA wide feature.\n:::\n::: feature title=Build icon=code layout=vertical width=half\nA compact feature.\n:::\n::: feature title=Test icon=false width=half\nNo icon.\n:::\n:::\n";
        let html = transform(source).unwrap();
        assert!(html.contains("workbook-features--grid"));
        assert!(html.contains("workbook-feature workbook-feature--horizontal workbook-feature--full"));
        assert!(html.contains("workbook-feature workbook-feature--vertical workbook-feature--half"));
        assert!(html.contains("workbook-feature workbook-feature--half"));
        assert_eq!(html.matches("workbook-feature__body").count(), 3);
    }

    #[test]
    fn mixed_features_support_card_height_spans() {
        let source = "::: features\n::: feature title=Tall width=half height=full layout=vertical\nTall body.\n:::\n::: feature title=Short width=half height=half layout=horizontal\nShort body.\n:::\n::: feature title=Default\nNormal body.\n:::\n:::\n";
        let html = transform(source).unwrap();
        assert!(html.contains("workbook-feature--half workbook-feature--height-full"));
        assert!(html.contains("workbook-feature--half workbook-feature--height-half"));
        assert!(html.contains("<div class=\"workbook-feature\">"));
        assert_eq!(html.matches("workbook-feature__body").count(), 3);
    }

    #[test]
    fn unknown_feature_heights_preserve_default() {
        let html = transform("::: features\n::: feature title=Normal height=large\nBody\n:::\n:::\n").unwrap();
        assert!(html.contains("<div class=\"workbook-feature\">"));
    }

    #[test]
    fn feature_layout_parent_remains_backwards_compatible() {
        let html = transform("::: features layout=horizontal\n::: feature title=Old\nBody\n:::\n:::\n").unwrap();
        assert!(html.contains("workbook-features--horizontal"));
        assert!(html.contains("class=\"workbook-feature\""));
    }

    #[test]
    fn horizontal_features_support_cards_without_icons() {
        let source = "::: features layout=horizontal\n::: feature title=First icon=false\nNo icon here.\n:::\n::: feature title=Second icon=terminal\nHas icon.\n:::\n:::\n";
        let html = transform(source).unwrap();
        assert!(html.contains("workbook-features--horizontal"));
        assert_eq!(html.matches("class=\"workbook-feature\"").count(), 2);
    }

    #[test]
    fn rejects_feature_without_features() {
        assert!(transform("::: feature title=Search\nText\n:::\n").is_err());
    }

    #[test]
    fn directives_inside_fenced_code_remain_literal() {
        let src = "```markdown\n::: hero module=ELEE1119\n:::\n```\n";
        assert_eq!(transform(src).unwrap(), src);
    }

    #[test]
    fn preserves_code_line_number_attributes() {
        let html = transform("```rust start=1 hl=2-3 linenumber=true\nlet x = 5;\n```\n").unwrap();
        assert!(html.contains("data-start=\"1\""));
        assert!(html.contains("data-hl=\"2-3\""));
        assert!(html.contains("data-ln=\"true\""));
    }

    #[test]
    fn expressive_metadata_is_preserved() {
        let src = "```rust showLineNumbers startLineNumber=5 hl=6 collapse={1-3,8-10} collapseStyle=collapsible-start\nlet x = 1;\n```\n";
        let html = transform(src).unwrap();
        assert!(html.contains("data-show-line-numbers=\"true\""));
        assert!(html.contains("data-start-line-number=\"5\""));
        assert!(html.contains("data-collapse=\"{1-3,8-10}\""));
        assert!(html.contains("data-collapse-style=\"collapsible-start\""));
    }

    #[test]
    fn legacy_code_metadata_remains_supported() {
        let src = "::: tabs\n::: tab label=Rust\n```rust start=9 ln=true hl=10\nlet x = 1;\n```\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("data-start=\"9\""));
        assert!(html.contains("data-ln=\"true\""));
        assert!(html.contains("data-hl=\"10\""));
    }

    #[test]
    fn code_metadata_is_escaped() {
        let src = "```rust collapseStyle='\" onmouseover=evil'\nlet x = 1;\n```\n";
        let html = transform(src).unwrap();
        assert!(!html.contains("onmouseover=evil\""));
    }

    #[test]
    fn danger_aside_renders_as_danger() {
        let html = transform("::: danger title=Security\nNever expose credentials.\n:::\n").unwrap();
        assert!(html.contains("mdc-aside--danger"));
        assert!(html.contains("Never expose credentials."));
    }

    #[test]
    fn complete_asides_example_parses() {
        let source = include_str!("../docs/src/EXAMPLE_2.md");
        let html = transform(source).expect("EXAMPLE_2.md must parse completely");
        for kind in ["note", "tip", "explanation", "success", "caution", "bug", "danger"] {
            assert!(html.contains(&format!("mdc-aside--{kind}")), "missing aside: {kind}");
        }
        assert!(html.contains("class=\"workbook-tabs\""));
    }

    #[test]
    fn tabs_render_labels_icons_and_sync_key() {
        let src = "::: tabs syncKey=platform\n::: tab label=Linux icon=terminal\nHello **Linux**.\n:::\n::: tab label=Windows icon=monitor\nHello Windows.\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("class=\"workbook-tabs\" data-sync-key=\"platform\""));
        assert_eq!(html.matches("class=\"workbook-tab-panel\"").count(), 2);
        assert!(html.contains("data-label=\"Linux\" data-icon=\"terminal\""));
        assert!(html.contains("<strong>Linux</strong>"));
    }

    #[test]
    fn tabs_preserve_code_fences_and_options() {
        let src = "::: tabs\n::: tab label=Rust\n```rust start=5 hl=6 linenumber=true\nfn main() {\n    println!(\"Hello\");\n}\n```\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("class=\"language-rust\""));
        assert!(html.contains("data-start=\"5\""));
        assert!(html.contains("data-hl=\"6\""));
    }

    #[test]
    fn tabs_allow_nested_callouts_and_steps() {
        let src = "::: tabs\n::: tab label=Guide\n::: tip\nTip text.\n:::\n::: steps\n1. First\n2. Second\n:::\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("mdc-aside--tip"));
        assert!(html.contains("class=\"workbook-steps\""));
        assert!(html.contains("class=\"workbook-tab-panel\""));
    }

    #[test]
    fn tabs_reject_invalid_structure() {
        for src in [
            "::: tab label=A\nHello\n:::\n",
            "::: tabs\n::: tab\nHello\n:::\n:::\n",
            "::: tabs\n:::\n",
            "::: tabs\nStray text\n:::\n",
            "::: tabs\n::: tab label=A\nA\n:::\n::: tab label=A\nB\n:::\n:::\n",
        ] {
            assert!(transform(src).is_err(), "accepted invalid input: {src}");
        }
    }

    #[test]
    fn tabs_escape_html_attributes() {
        let src = "::: tabs syncKey='<unsafe>'\n::: tab label='A & B' icon='\" onclick=\"alert(1)'\nSafe\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("data-sync-key=\"&lt;unsafe&gt;\""));
        assert!(html.contains("data-label=\"A &amp; B\""));
        assert!(!html.contains("onclick=\"alert(1)\""));
    }

    #[test]
    fn directives_inside_tab_fence_are_not_parsed() {
        let src = "::: tabs\n::: tab label=Markdown\n```md\n::: tip\nNot a component\n:::\n```\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(!html.contains("mdc-aside--tip"));
        assert!(html.contains("::: tip"));
    }


    #[test]
    fn new_aside_variants_have_distinct_semantics_and_icons() {
        for kind in ["success", "explanation", "bug"] {
            let src = format!("::: {kind} title=Sample\nBody\n:::\n");
            let html = transform(&src).unwrap();
            assert!(html.contains(&format!("mdc-aside--{kind}")), "{kind}: {html}");
            assert!(html.contains(&format!("data-kind=\"{kind}\"")), "{kind}: {html}");
            assert!(html.contains("mdc-aside__icon"));
        }
    }

    #[test]
    fn new_aside_variants_support_collapsing_and_icon_false() {
        let html = transform("::: bug title=Diagnosis collapsible=true open=true icon=false\nTest\n:::\n").unwrap();
        assert!(html.contains("<details class=\"mdc-aside mdc-aside--bug\""));
        assert!(html.contains(" open>"));
        assert!(!html.contains("mdc-aside__icon"));
    }

    #[test]
    fn chrome_appends_after_gitinfo_blocks_without_nesting() {
        let chrome = ChapterChrome {
            header: Some("<header class=\"workbook-header\">📘 Module</header>\n\n".to_owned()),
            footer: Some("<footer class=\"workbook-footer\">🦀 Course</footer>\n\n".to_owned()),
        };
        let input = "<header class=\"gitinfo-header\">Git metadata</header>\n\n# Chapter\n\n<footer class=\"gitinfo-footer\">Commit metadata</footer>\n";
        let output = crate::book::combine_chrome_for_tests(input, &chrome);
        let gh = output.find("gitinfo-header").unwrap();
        let wh = output.find("workbook-header").unwrap();
        let title = output.find("# Chapter").unwrap();
        let gf = output.find("gitinfo-footer").unwrap();
        let wf = output.find("workbook-footer").unwrap();
        assert!(gh < wh && wh < title && title < gf && gf < wf, "{output}");
        assert_eq!(output.matches("<header").count(), 2);
        assert_eq!(output.matches("<footer").count(), 2);
        assert_eq!(output.matches("</header>").count(), 2);
        assert_eq!(output.matches("</footer>").count(), 2);
        assert!(output.contains("Git metadata"));
        assert!(output.contains("Commit metadata"));
    }

    #[test]
    fn chrome_is_not_duplicated_if_already_inserted() {
        let chrome = ChapterChrome {
            header: Some("<header class=\"workbook-header\">📘</header>\n\n".to_owned()),
            footer: Some("<footer class=\"workbook-footer\">🦀</footer>\n\n".to_owned()),
        };
        let once = crate::book::combine_chrome_for_tests("# Chapter\n", &chrome);
        let twice = crate::book::combine_chrome_for_tests(&once, &chrome);
        assert_eq!(once, twice);
        assert_eq!(twice.matches("workbook-header").count(), 1);
        assert_eq!(twice.matches("workbook-footer").count(), 1);
    
    #[test]
    fn arbitrary_nested_asides_render_to_three_levels() {
        let src = "::: note title=Outer\nOuter description.\n\n::: important title=Middle\nMiddle description.\n\n::: example title=Inner\n**Inner body**\n:::\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("mdc-aside--note"), "{html}");
        assert!(html.contains("mdc-aside--important"), "{html}");
        assert!(html.contains("mdc-aside--example"), "{html}");
        assert!(html.contains("<strong>Inner body</strong>"), "{html}");
        assert!(!html.contains("MDCNESTEDBLOCK"), "{html}");
    }

    #[test]
    fn named_closers_report_exact_mismatch_lines() {
        let error = transform("::: note\n::: tip\nContent\n::: /note\n").unwrap_err();
        assert!(error.contains("line 4"), "{error}");
        assert!(error.contains("tip opened at line 2"), "{error}");
        let error = transform("::: note\n::: tip\nContent\n:::\n").unwrap_err();
        assert!(error.contains("note (line 1)"), "{error}");
        assert!(error.contains("line 5"), "{error}");
    }

    #[test]
    fn image_component_validates_source_and_renders_dimensions() {
        let html = transform("::: image src='./figures/example.png' width=75% height=240 align=right alt='Circuit diagram'\nAn example image.\n:::\n").unwrap();
        assert!(html.contains("mdc-image--right"), "{html}");
        assert!(html.contains("--mdc-image-width: 75%;"), "{html}");
        assert!(html.contains("--mdc-image-height: 240px;"), "{html}");
        assert!(html.contains("alt=\"Circuit diagram\""), "{html}");
        assert!(html.contains("<figcaption>"), "{html}");
        assert!(transform("::: image width=50%\n:::\n").is_err());
        let html = transform("::: image src='javascript:alert(1)'\n:::\n").unwrap();
        assert!(!html.contains("<img"));
    }

    #[test]
    fn hero_without_columns_is_full_width_layout() {
        let html = transform("::: hero heading=Welcome\nOne paragraph.\n:::\n").unwrap();
        assert!(html.contains("class=\"workbook-hero\""), "{html}");
        assert!(!html.contains("workbook-hero--columns"), "{html}");
    }
    #[test]
    fn raw_html_does_not_leak_nested_component_placeholders() {
        let src = "::: hero heading=Demo\n<a href=\"https://example.org\">Example</a><br>\n::: note title=Info\nNested body\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("mdc-aside--note"), "{html}");
        assert!(html.contains("Nested body"), "{html}");
        assert!(!html.contains("MDCNESTEDBLOCK"), "{html}");
    }

    #[test]
    fn ordered_feature_columns_preserve_card_order() {
        let src = "::: features layout=columns\n::: feature title=A width=half height=half\nFirst\n:::\n::: feature title=B width=half height=half\nSecond\n:::\n::: feature title=C width=half height=full\nThird\n:::\n:::\n";
        let html = transform(src).unwrap();
        assert!(html.contains("workbook-features--columns"), "{html}");
        assert_eq!(html.matches("workbook-feature--half").count(), 3);
        assert!(html.find("First").unwrap() < html.find("Second").unwrap());
        assert!(html.find("Second").unwrap() < html.find("Third").unwrap());
        let grid = transform("::: features\n::: feature title=A\nHello\n:::\n:::\n").unwrap();
        assert!(grid.contains("workbook-features--grid"));
    }

    #[test]
    fn unknown_component_inside_tabs_reports_original_line_with_suggestion() {
        let src = "::: tabs\n::: tab label=Guide\n::: importnat\nRead this.\n:::\n::: explanation\nWhy it matters.\n:::\n:::\n:::\n";
        let err = transform(src).unwrap_err();
        assert!(err.contains("Unknown component '::: importnat' at line 3"), "{err}");
        assert!(err.contains("did you mean '::: important'?"), "{err}");
    }

    #[test]
    fn unknown_component_at_root_is_reported() {
        let err = transform("::: notacomponent\nSomething\n:::\n").unwrap_err();
        assert!(err.contains("Unknown component '::: notacomponent' at line 1"), "{err}");
    }

    #[test]
    fn unknown_component_like_text_in_fence_is_ignored() {
        let src = "```md\n::: importnat\n```\n";
        assert_eq!(transform(src).unwrap(), src);
    }
}

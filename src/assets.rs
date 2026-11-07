// Extracted from the original main.rs without changing the implementation.
use std::{error::Error, fs, path::PathBuf};
use toml_edit::{value, Array, DocumentMut, Item};

pub(crate) const COMPONENT_CSS: &str = include_str!("../theme/css/mdbook-components.css");
pub(crate) const DEVICON_CSS: &str = include_str!("../theme/css/mdbook-components-devicon.css");
pub(crate) const COMPONENT_JS: &str = include_str!("../theme/js/mdbook-components.js");
pub(crate) const COMPONENT_ICONS_JS: &str = include_str!("../theme/js/mdbook-components-icons.js");

pub(crate) fn init_usage() {
    println!("mdbook-components init [--book-dir PATH] [--css-dir PATH] [--js-dir PATH] [--force]\n\
         Defaults: --book-dir . --css-dir theme/css --js-dir theme/js\n\
         CSS and JS directories are relative to the directory containing book.toml.\n\
         --force overwrites existing component assets that differ from the bundled versions.\n\
         Other additional-css and additional-js entries are retained.");
}

pub(crate) fn valid_asset_dir(path: &std::path::Path) -> bool {
    !path.is_absolute()
        && path.components().all(|part| matches!(
            part,
            std::path::Component::Normal(_) | std::path::Component::CurDir
        ))
}

pub(crate) fn register_asset(
    config: &mut DocumentMut,
    setting: &str,
    relative_path: &std::path::Path,
) -> Result<(), Box<dyn Error>> {
    let entry = relative_path
        .to_str()
        .ok_or("Asset path must be valid UTF-8")?
        .replace('\\', "/");
    let setting_item = &mut config["output"]["html"][setting];
    if setting_item.is_none() {
        *setting_item = value(Array::new());
    }
    let array = setting_item
        .as_array_mut()
        .ok_or_else(|| format!("[output.html].{setting} must be an array"))?;
    if !array.iter().any(|v| v.as_str() == Some(entry.as_str())) {
        array.push(entry);
    }
    Ok(())
}

pub(crate) fn check_asset(
    root: &std::path::Path,
    relative_dir: &std::path::Path,
    filename: &str,
    bundled: &str,
    force: bool,
) -> Result<PathBuf, Box<dyn Error>> {
    let directory = root.join(relative_dir);
    // Create directories before checking canonical paths so symlinks cannot
    // direct writes outside the book root.
    fs::create_dir_all(&directory)?;
    if !fs::canonicalize(&directory)?.starts_with(root) {
        return Err(format!("Asset directory resolves outside book root: {}", directory.display()).into());
    }
    let destination = directory.join(filename);
    if destination.exists() && !force && fs::read_to_string(&destination)? != bundled {
        return Err(format!("{} already exists with different content; use --force to replace it", destination.display()).into());
    }
    Ok(destination)
}

pub(crate) fn init(args: &[String]) -> Result<(), Box<dyn Error>> {
    let mut book_dir = PathBuf::from(".");
    let mut css_dir = PathBuf::from("theme/css");
    let mut js_dir = PathBuf::from("theme/js");
    let mut force = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => { init_usage(); return Ok(()); }
            "--book-dir" | "--css-dir" | "--js-dir" => {
                let flag = args[i].as_str();
                i += 1;
                let val = args.get(i).ok_or_else(|| format!("Missing value for {flag}"))?;
                match flag {
                    "--book-dir" => book_dir = PathBuf::from(val),
                    "--css-dir" => css_dir = PathBuf::from(val),
                    "--js-dir" => js_dir = PathBuf::from(val),
                    _ => unreachable!(),
                }
            }
            "--force" => force = true,
            arg => return Err(format!("Unknown init argument: {arg}").into()),
        }
        i += 1;
    }
    if !valid_asset_dir(&css_dir) || !valid_asset_dir(&js_dir) {
        return Err("--css-dir and --js-dir must be relative directories within the book root".into());
    }

    let root = fs::canonicalize(&book_dir)?;
    let toml_path = root.join("book.toml");
    let old_config = fs::read_to_string(&toml_path)?;
    let mut config = old_config.parse::<DocumentMut>()?;

    if config.get("preprocessor").is_none() {
        config["preprocessor"] = Item::Table(toml_edit::Table::new());
    }
    if config["preprocessor"].get("components").is_none() {
        config["preprocessor"]["components"] = Item::Table(toml_edit::Table::new());
    }
    config["preprocessor"]["components"]["renderers"] = value(Array::from_iter(["html"]));
    if config["preprocessor"].get("track").is_some() {
        config["preprocessor"]["components"]["after"] = value(Array::from_iter(["track"]));
    } else if let Some(table) = config["preprocessor"]["components"].as_table_mut() {
        table.remove("after");
    }

    if config.get("output").is_none() {
        config["output"] = Item::Table(toml_edit::Table::new());
    }
    if config["output"].get("html").is_none() {
        config["output"]["html"] = Item::Table(toml_edit::Table::new());
    }
    // Replace only old mdbook-components asset registrations, retaining
    // third-party assets and preserving the icon script's load order.
    for (setting, own_files) in [
        ("additional-css", &["components.css", "starlight-extras.css", "mdbook-components.css", "mdbook-components-devicon.css"][..]),
        ("additional-js", &["components.js", "filetree-icons.js", "tabs.js", "mdbook-components.js", "mdbook-components-icons.js"][..]),
    ] {
        if let Some(entries) = config["output"]["html"][setting].as_array_mut() {
            let obsolete: Vec<String> = own_files.iter()
                .map(|name| {
                    let dir = if setting == "additional-css" { &css_dir } else { &js_dir };
                    dir.join(name).to_string_lossy().replace('\\', "/")
                })
                .collect();
            for idx in (0..entries.len()).rev() {
                if entries.get(idx).and_then(|e| e.as_str())
                    .is_some_and(|s| obsolete.iter().any(|name| name == s)) {
                    entries.remove(idx);
                }
            }
        }
    }
    register_asset(&mut config, "additional-css", &css_dir.join("mdbook-components-devicon.css"))?;
    register_asset(&mut config, "additional-css", &css_dir.join("mdbook-components.css"))?;
    register_asset(&mut config, "additional-js", &js_dir.join("mdbook-components-icons.js"))?;
    register_asset(&mut config, "additional-js", &js_dir.join("mdbook-components.js"))?;

    // Validate both destinations before overwriting either file.
    let devicon_css_path = check_asset(&root, &css_dir, "mdbook-components-devicon.css", DEVICON_CSS, force)?;
    let css_path = check_asset(&root, &css_dir, "mdbook-components.css", COMPONENT_CSS, force)?;
    let js_path = check_asset(&root, &js_dir, "mdbook-components.js", COMPONENT_JS, force)?;
    let icons_js_path = check_asset(&root, &js_dir, "mdbook-components-icons.js", COMPONENT_ICONS_JS, force)?;

    if !devicon_css_path.exists() || fs::read_to_string(&devicon_css_path)? != DEVICON_CSS {
        fs::write(&devicon_css_path, DEVICON_CSS)?;
    }
    if !css_path.exists() || fs::read_to_string(&css_path)? != COMPONENT_CSS {
        fs::write(&css_path, COMPONENT_CSS)?;
    }
    if !js_path.exists() || fs::read_to_string(&js_path)? != COMPONENT_JS {
        fs::write(&js_path, COMPONENT_JS)?;
    }
    if !icons_js_path.exists() || fs::read_to_string(&icons_js_path)? != COMPONENT_ICONS_JS {
        fs::write(&icons_js_path, COMPONENT_ICONS_JS)?;
    }
    let new_config = config.to_string();
    if old_config != new_config {
        fs::write(&toml_path, new_config)?;
    }
    println!("Installed Devicon stylesheet: {}", devicon_css_path.display());
    println!("Installed stylesheet: {}", css_path.display());
    println!("Installed icon script: {}", icons_js_path.display());
    println!("Installed script: {}", js_path.display());
    println!("Configured: {}", toml_path.display());
    println!("Run: mdbook build");
    Ok(())
}


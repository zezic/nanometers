use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=rainglow-themes/themes/");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("rainglow_themes.rs");

    let mut themes = HashMap::new();

    // Process all theme files
    if let Ok(entries) = fs::read_dir("rainglow-themes/themes") {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "json" {
                    if let Some(file_name) = entry.path().file_stem() {
                        let theme_name = file_name.to_string_lossy().to_string();

                        // Read and parse the theme file
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if let Ok(theme_json) =
                                serde_json::from_str::<serde_json::Value>(&content)
                            {
                                if let Some(colors) = theme_json.get("colors") {
                                    themes.insert(theme_name, colors.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Generate the code
    let mut themes_code = String::new();
    themes_code.push_str("use serde_json::Value;\n");
    themes_code.push_str("use once_cell::sync::Lazy;\n\n");

    themes_code
        .push_str("pub static RAINGLOW_THEMES: Lazy<std::collections::HashMap<String, Value>> = Lazy::new(|| {\n");
    themes_code.push_str("    let mut themes = std::collections::HashMap::new();\n");

    for (name, colors) in themes {
        let colors_str = serde_json::to_string(&colors).unwrap();
        // Escape the quotes in the JSON string for embedding in Rust code
        let escaped_colors = colors_str.replace('\\', "\\\\").replace('"', "\\\"");
        themes_code.push_str(&format!(
            "    themes.insert(\"{}\".to_string(), serde_json::from_str(\"{}\").unwrap());\n",
            name, escaped_colors
        ));
    }

    themes_code.push_str("    themes\n");
    themes_code.push_str("});\n");

    fs::write(&dest_path, themes_code).unwrap();
}

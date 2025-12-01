use crate::setting::Theme;
use egui::Color32;
use serde::{Deserialize, Serialize};


// Include the generated Rainglow themes at compile time
include!(concat!(env!("OUT_DIR"), "/rainglow_themes.rs"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RainglowMapping {
    pub main: String,
    pub bg: String,
    pub bgaccent: String,
    pub text: String,
    pub accent: String,
    pub frame: String,
    pub selection: String,
    pub spectrum_main: String,
    pub spectrum_secondary: String,
    pub spectrum_ref_line: String,
}

impl Default for RainglowMapping {
    fn default() -> Self {
        Self {
            main: "activityBarBadge.background".to_string(),
            bg: "editor.background".to_string(),
            bgaccent: "sideBar.background".to_string(),
            text: "editor.foreground".to_string(),
            accent: "list.activeSelectionBackground".to_string(),
            frame: "editorIndentGuide.background".to_string(),
            selection: "editor.selectionBackground".to_string(),
            spectrum_main: "activityBarBadge.background".to_string(),
            spectrum_secondary: "statusBar.background".to_string(),
            spectrum_ref_line: "editorLineNumber.foreground".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RainglowThemeManager {
    mapping: RainglowMapping,
}

impl RainglowThemeManager {
    pub fn new() -> Self {
        Self {
            mapping: RainglowMapping::default(),
        }
    }

    pub fn with_mapping(mapping: RainglowMapping) -> Self {
        Self { mapping }
    }

    pub fn get_mapping(&self) -> &RainglowMapping {
        &self.mapping
    }

    pub fn set_mapping(&mut self, mapping: RainglowMapping) {
        self.mapping = mapping;
    }

    pub fn get_theme_names(&self) -> Vec<String> {
        let mut names: Vec<String> = RAINGLOW_THEMES.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn convert_theme(&self, theme_name: &str) -> Option<Theme> {
        if let Some(colors) = RAINGLOW_THEMES.get(theme_name) {
            let mut theme = Theme {
                name: format!("Rainglow: {}", theme_name),
                ..Default::default()
            };

            // Apply mapping
            theme.main = self.get_color(colors, &self.mapping.main);
            theme.bg = self.get_color(colors, &self.mapping.bg);
            theme.bgaccent = self.get_color(colors, &self.mapping.bgaccent);
            theme.text = self.get_color(colors, &self.mapping.text);
            theme.accent = self.get_color(colors, &self.mapping.accent);
            theme.frame = self.get_color(colors, &self.mapping.frame);
            theme.selection = self.get_color(colors, &self.mapping.selection);
            theme.spectrum_main = self.get_color(colors, &self.mapping.spectrum_main);
            theme.spectrum_secondary = self.get_color(colors, &self.mapping.spectrum_secondary);
            theme.spectrum_ref_line = self.get_color(colors, &self.mapping.spectrum_ref_line);

            Some(theme)
        } else {
            None
        }
    }

    fn get_color(&self, colors: &serde_json::Value, key: &str) -> Color32 {
        if let Some(color_str) = colors.get(key).and_then(|v| v.as_str()) {
            parse_hex_color(color_str)
        } else {
            // Fallback colors if key doesn't exist
            match key.contains("background") || key.contains("Background") {
                true => Color32::from_rgb(50, 50, 50),     // Dark fallback
                false => Color32::from_rgb(200, 200, 200), // Light fallback
            }
        }
    }

    pub fn get_available_color_keys(&self, theme_name: &str) -> Vec<String> {
        if let Some(colors) = RAINGLOW_THEMES.get(theme_name) {
            if let Some(obj) = colors.as_object() {
                let mut keys: Vec<String> = obj.keys().cloned().collect();
                keys.sort();
                return keys;
            }
        }
        Vec::new()
    }
}

impl Default for RainglowThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_hex_color(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');

    // Handle 8-digit hex (with alpha)
    if hex.len() == 8 {
        if let Ok(rgba) = u32::from_str_radix(hex, 16) {
            let r = ((rgba >> 24) & 0xFF) as u8;
            let g = ((rgba >> 16) & 0xFF) as u8;
            let b = ((rgba >> 8) & 0xFF) as u8;
            let a = (rgba & 0xFF) as u8;
            return Color32::from_rgba_unmultiplied(r, g, b, a);
        }
    }

    // Handle 6-digit hex (no alpha)
    if hex.len() == 6 {
        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            return Color32::from_rgb(r, g, b);
        }
    }

    // Handle 3-digit hex (short form)
    if hex.len() == 3 {
        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = ((rgb >> 8) & 0xF) as u8;
            let g = ((rgb >> 4) & 0xF) as u8;
            let b = (rgb & 0xF) as u8;
            // Expand 4-bit to 8-bit
            let r = r * 17; // 0xF -> 0xFF
            let g = g * 17;
            let b = b * 17;
            return Color32::from_rgb(r, g, b);
        }
    }

    // Fallback to white if parsing fails
    Color32::WHITE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_parsing() {
        assert_eq!(parse_hex_color("#FF0000"), Color32::from_rgb(255, 0, 0));
        assert_eq!(parse_hex_color("#00FF00"), Color32::from_rgb(0, 255, 0));
        assert_eq!(parse_hex_color("#0000FF"), Color32::from_rgb(0, 0, 255));
        assert_eq!(parse_hex_color("#FFFFFF"), Color32::from_rgb(255, 255, 255));
        assert_eq!(parse_hex_color("FF0000"), Color32::from_rgb(255, 0, 0)); // No #
        assert_eq!(parse_hex_color("#F00"), Color32::from_rgb(255, 0, 0)); // Short form
    }

    #[test]
    fn test_default_mapping() {
        let mapping = RainglowMapping::default();
        assert_eq!(mapping.bg, "editor.background");
        assert_eq!(mapping.text, "editor.foreground");
    }

    #[test]
    fn test_rainglow_themes_loaded() {
        let manager = RainglowThemeManager::new();
        let theme_names = manager.get_theme_names();

        // Should have many themes loaded
        assert!(theme_names.len() > 50, "Expected many Rainglow themes, got {}", theme_names.len());

        // Check some expected themes exist
        assert!(theme_names.contains(&"darkside".to_string()));
        assert!(theme_names.contains(&"rainbow".to_string()));
    }

    #[test]
    fn test_theme_conversion() {
        let manager = RainglowThemeManager::new();

        // Test converting a specific theme
        if let Some(theme) = manager.convert_theme("darkside") {
            assert_eq!(theme.name, "Rainglow: darkside");
            // Colors should be valid (not default fallbacks)
            assert_ne!(theme.bg, Color32::from_rgb(50, 50, 50));
        } else {
            panic!("Failed to convert darkside theme");
        }
    }

    #[test]
    fn test_available_color_keys() {
        let manager = RainglowThemeManager::new();
        let keys = manager.get_available_color_keys("darkside");

        // Should have many color keys
        assert!(keys.len() > 10, "Expected many color keys, got {}", keys.len());

        // Check some expected keys exist
        assert!(keys.contains(&"editor.background".to_string()));
        assert!(keys.contains(&"editor.foreground".to_string()));
        assert!(keys.contains(&"statusBar.background".to_string()));
    }
}

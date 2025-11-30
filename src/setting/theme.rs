use crate::NanometersApp;
use directories::ProjectDirs;
use egui::style::*;
use egui::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub(crate) main: Color32,
    pub(crate) bg: Color32,
    pub(crate) bgaccent: Color32,
    pub(crate) text: Color32,
    pub(crate) accent: Color32,
    pub(crate) frame: Color32,
    pub(crate) selection: Color32,
    pub(crate) spectrum_main: Color32,
    pub(crate) spectrum_secondary: Color32,
    pub(crate) spectrum_ref_line: Color32,
}

#[derive(Debug, Clone)]
pub struct ThemeManager {
    pub themes: HashMap<String, Theme>,
    pub current_theme_name: String,
    config_dir: PathBuf,
}

pub fn create_dark_theme() -> Theme {
    Theme {
        name: "Dark".to_string(),
        main: Color32::from_rgb(172, 192, 222),
        bg: Color32::from_rgb(43, 48, 55),
        bgaccent: Color32::from_rgb(0x3b, 0x3b, 0x3b),
        text: Color32::from_rgb(0xf0, 0xf0, 0xea),
        accent: Color32::from_rgb(0x6f, 0x6f, 0xff),
        frame: Color32::from_rgb(65, 65, 65),
        selection: Color32::from_rgb(172, 192, 222),
        spectrum_main: Color32::from_rgb(172, 192, 222),
        spectrum_secondary: Color32::from_rgb(255, 160, 122),
        spectrum_ref_line: Color32::from_rgb(100, 100, 100),
    }
}

pub fn create_light_theme() -> Theme {
    Theme {
        name: "Light".to_string(),
        main: Color32::from_rgb(0x58, 0x56, 0xcf),
        bg: Color32::from_rgb(240, 240, 234),
        bgaccent: Color32::from_rgb(224, 224, 234),
        text: Color32::from_rgb(43, 43, 43),
        accent: Color32::from_rgb(111, 111, 255),
        frame: Color32::from_rgb(190, 190, 190),
        selection: Color32::from_rgb(189, 189, 229),
        spectrum_main: Color32::from_rgb(0x58, 0x56, 0xcf),
        spectrum_secondary: Color32::from_rgb(255, 100, 0),
        spectrum_ref_line: Color32::from_rgb(120, 120, 120),
    }
}

pub fn create_pink_theme() -> Theme {
    Theme {
        name: "Pink".to_string(),
        main: Color32::from_rgb(255, 255, 255),
        bg: Color32::from_rgb(255, 192, 203),
        bgaccent: Color32::from_rgb(243, 202, 203),
        text: Color32::from_rgb(255, 255, 255),
        accent: Color32::from_rgb(255, 233, 203),
        frame: Color32::from_rgb(255, 255, 255),
        selection: Color32::from_rgb(233, 233, 203),
        spectrum_main: Color32::from_rgb(255, 255, 255),
        spectrum_secondary: Color32::from_rgb(255, 20, 147),
        spectrum_ref_line: Color32::from_rgb(200, 200, 200),
    }
}

pub fn create_cyberpunk_theme() -> Theme {
    Theme {
        name: "Cyberpunk".to_string(),
        main: Color32::from_rgb(0, 255, 255),
        bg: Color32::from_rgb(13, 13, 23),
        bgaccent: Color32::from_rgb(25, 25, 40),
        text: Color32::from_rgb(0, 255, 255),
        accent: Color32::from_rgb(255, 20, 147),
        frame: Color32::from_rgb(75, 0, 130),
        selection: Color32::from_rgb(255, 20, 147),
        spectrum_main: Color32::from_rgb(0, 255, 255),
        spectrum_secondary: Color32::from_rgb(255, 20, 147),
        spectrum_ref_line: Color32::from_rgb(75, 0, 130),
    }
}

pub fn create_ocean_theme() -> Theme {
    Theme {
        name: "Ocean".to_string(),
        main: Color32::from_rgb(64, 224, 208),
        bg: Color32::from_rgb(25, 25, 112),
        bgaccent: Color32::from_rgb(70, 130, 180),
        text: Color32::from_rgb(240, 248, 255),
        accent: Color32::from_rgb(0, 191, 255),
        frame: Color32::from_rgb(100, 149, 237),
        selection: Color32::from_rgb(64, 224, 208),
        spectrum_main: Color32::from_rgb(64, 224, 208),
        spectrum_secondary: Color32::from_rgb(0, 191, 255),
        spectrum_ref_line: Color32::from_rgb(70, 130, 180),
    }
}

pub fn create_matrix_theme() -> Theme {
    Theme {
        name: "Matrix".to_string(),
        main: Color32::from_rgb(0, 255, 0),
        bg: Color32::from_rgb(0, 0, 0),
        bgaccent: Color32::from_rgb(10, 20, 10),
        text: Color32::from_rgb(0, 255, 0),
        accent: Color32::from_rgb(50, 255, 50),
        frame: Color32::from_rgb(0, 100, 0),
        selection: Color32::from_rgb(0, 150, 0),
        spectrum_main: Color32::from_rgb(0, 255, 0),
        spectrum_secondary: Color32::from_rgb(255, 0, 0),
        spectrum_ref_line: Color32::from_rgb(0, 150, 0),
    }
}

// Backward compatibility constants
pub const DARK_THEME: Theme = Theme {
    name: String::new(),
    main: Color32::from_rgb(172, 192, 222),
    bg: Color32::from_rgb(43, 48, 55),
    bgaccent: Color32::from_rgb(0x3b, 0x3b, 0x3b),
    text: Color32::from_rgb(0xf0, 0xf0, 0xea),
    accent: Color32::from_rgb(0x6f, 0x6f, 0xff),
    frame: Color32::from_rgb(65, 65, 65),
    selection: Color32::from_rgb(172, 192, 222),
    spectrum_main: Color32::from_rgb(172, 192, 222),
    spectrum_secondary: Color32::from_rgb(255, 160, 122),
    spectrum_ref_line: Color32::from_rgb(100, 100, 100),
};

pub const LIGHT_THEME: Theme = Theme {
    name: String::new(),
    main: Color32::from_rgb(0x58, 0x56, 0xcf),
    bg: Color32::from_rgb(240, 240, 234),
    bgaccent: Color32::from_rgb(224, 224, 234),
    text: Color32::from_rgb(43, 43, 43),
    accent: Color32::from_rgb(111, 111, 255),
    frame: Color32::from_rgb(190, 190, 190),
    selection: Color32::from_rgb(189, 189, 229),
    spectrum_main: Color32::from_rgb(0x58, 0x56, 0xcf),
    spectrum_secondary: Color32::from_rgb(255, 100, 0),
    spectrum_ref_line: Color32::from_rgb(120, 120, 120),
};

pub const PINK_THEME: Theme = Theme {
    name: String::new(),
    main: Color32::from_rgb(255, 255, 255),
    bg: Color32::from_rgb(255, 192, 203),
    bgaccent: Color32::from_rgb(243, 202, 203),
    text: Color32::from_rgb(255, 255, 255),
    accent: Color32::from_rgb(255, 233, 203),
    frame: Color32::from_rgb(255, 255, 255),
    selection: Color32::from_rgb(233, 233, 203),
    spectrum_main: Color32::from_rgb(255, 255, 255),
    spectrum_secondary: Color32::from_rgb(255, 20, 147),
    spectrum_ref_line: Color32::from_rgb(200, 200, 200),
};

impl ThemeManager {
    pub fn new() -> Self {
        let config_dir = Self::get_themes_dir();
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme_name: "Dark".to_string(),
            config_dir: config_dir.clone(),
        };

        // Ensure config directory exists
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("Warning: Could not create themes directory: {}", e);
        }

        manager.load_builtin_themes();
        manager.load_custom_themes();
        manager
    }

    fn get_themes_dir() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "Nanometers") {
            proj_dirs.data_dir().join("themes")
        } else {
            // Fallback to local directory
            PathBuf::from("config/themes")
        }
    }

    fn load_builtin_themes(&mut self) {
        self.themes.insert("Dark".to_string(), create_dark_theme());
        self.themes
            .insert("Light".to_string(), create_light_theme());
        self.themes.insert("Pink".to_string(), create_pink_theme());
        self.themes
            .insert("Cyberpunk".to_string(), create_cyberpunk_theme());
        self.themes
            .insert("Ocean".to_string(), create_ocean_theme());
        self.themes
            .insert("Matrix".to_string(), create_matrix_theme());
    }

    fn load_custom_themes(&mut self) {
        if let Ok(entries) = fs::read_dir(&self.config_dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "json" {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if let Ok(theme) = serde_json::from_str::<Theme>(&content) {
                                self.themes.insert(theme.name.clone(), theme);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    pub fn get_current_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.current_theme_name)
    }

    pub fn set_current_theme(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.current_theme_name = name.to_string();
            true
        } else {
            false
        }
    }

    pub fn save_theme(&mut self, theme: Theme) -> Result<(), Box<dyn std::error::Error>> {
        // Protect built-in themes from being overwritten
        if self.is_builtin_theme(&theme.name) {
            return Err("Cannot overwrite built-in themes".into());
        }

        let file_path = self.config_dir.join(format!("{}.json", theme.name));
        let content = serde_json::to_string_pretty(&theme)?;
        fs::write(file_path, content)?;
        self.themes.insert(theme.name.clone(), theme);
        Ok(())
    }

    pub fn save_theme_as_copy(
        &mut self,
        original_name: &str,
        new_name: &str,
    ) -> Result<Theme, Box<dyn std::error::Error>> {
        if self.themes.contains_key(new_name) {
            return Err("Theme with new name already exists".into());
        }

        if let Some(original_theme) = self.themes.get(original_name) {
            let mut new_theme = original_theme.clone();
            new_theme.name = new_name.to_string();
            self.save_theme(new_theme.clone())?;
            Ok(new_theme)
        } else {
            Err("Original theme not found".into())
        }
    }

    pub fn delete_theme(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_builtin_theme(name) {
            return Err("Cannot delete built-in themes".into());
        }

        let file_path = self.config_dir.join(format!("{}.json", name));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        self.themes.remove(name);

        // If we deleted the current theme, switch to Dark
        if self.current_theme_name == name {
            self.current_theme_name = "Dark".to_string();
        }
        Ok(())
    }

    pub fn rename_theme(
        &mut self,
        old_name: &str,
        new_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_builtin_theme(old_name) {
            return Err("Cannot rename built-in themes".into());
        }

        if self.themes.contains_key(new_name) {
            return Err("Theme with new name already exists".into());
        }

        if let Some(mut theme) = self.themes.remove(old_name) {
            // Delete old file
            let old_file_path = self.config_dir.join(format!("{}.json", old_name));
            if old_file_path.exists() {
                fs::remove_file(old_file_path)?;
            }

            // Update theme name and save with new name
            theme.name = new_name.to_string();
            self.save_theme(theme)?;

            // Update current theme name if needed
            if self.current_theme_name == old_name {
                self.current_theme_name = new_name.to_string();
            }
        } else {
            return Err("Theme not found".into());
        }
        Ok(())
    }

    pub fn create_new_theme(&mut self, name: &str) -> Result<Theme, Box<dyn std::error::Error>> {
        if self.themes.contains_key(name) {
            return Err("Theme with this name already exists".into());
        }

        let mut new_theme = create_dark_theme();
        new_theme.name = name.to_string();
        self.save_theme(new_theme.clone())?;
        Ok(new_theme)
    }

    pub fn get_theme_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.themes.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn is_builtin_theme(&self, name: &str) -> bool {
        matches!(
            name,
            "Dark" | "Light" | "Pink" | "Cyberpunk" | "Ocean" | "Matrix"
        )
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn set_theme(app: &mut NanometersApp) -> Visuals {
    Visuals {
        dark_mode: false,
        override_text_color: Some(app.setting.theme.text),
        selection: Selection {
            bg_fill: app.setting.theme.selection,
            stroke: Stroke::NONE,
        },
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: app.setting.theme.bgaccent,
                weak_bg_fill: app.setting.theme.bgaccent,
                bg_stroke: Stroke::new(1.0, app.setting.theme.frame),
                rounding: 0.0.into(),
                fg_stroke: Stroke::NONE,
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                bg_fill: app.setting.theme.bgaccent,
                weak_bg_fill: app.setting.theme.bgaccent,
                bg_stroke: Stroke::NONE,
                rounding: 0.0.into(),
                fg_stroke: Stroke::NONE,
                expansion: 0.0,
            },
            active: WidgetVisuals {
                bg_fill: app.setting.theme.selection,
                weak_bg_fill: app.setting.theme.selection,
                bg_stroke: Stroke::NONE,
                rounding: 0.0.into(),
                fg_stroke: Stroke::NONE,
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                bg_fill: app.setting.theme.selection,
                weak_bg_fill: app.setting.theme.selection,
                bg_stroke: Stroke::NONE,
                rounding: 0.0.into(),
                fg_stroke: Stroke::new(1.0, app.setting.theme.text),
                expansion: 0.0,
            },
            open: WidgetVisuals {
                bg_fill: app.setting.theme.bg,
                weak_bg_fill: app.setting.theme.bgaccent,
                bg_stroke: Stroke::NONE,
                rounding: 0.0.into(),
                fg_stroke: Stroke::NONE,
                expansion: 0.0,
            },
        },
        ..Default::default()
    }
}

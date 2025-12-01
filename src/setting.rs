use serde::{Deserialize, Serialize};

pub(crate) mod audio_device;
pub(crate) mod audio_source;
pub(crate) mod meter;
pub(crate) mod oscilloscope;
pub(crate) mod peak;
pub(crate) mod rainglow;
pub(crate) mod spectrogram;
pub(crate) mod spectrum;
pub(crate) mod theme;
pub(crate) mod vectorscope;
pub(crate) mod waveform;

pub use audio_device::*;
pub use audio_source::*;
pub use meter::*;
pub use oscilloscope::*;
pub use peak::*;
pub use rainglow::*;
pub use spectrogram::*;
pub use spectrum::*;
pub use theme::*;
pub use vectorscope::*;
pub use waveform::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub audio_device: AudioDeviceSetting,
    pub waveform: WaveformSetting,
    pub spectrogram: SpectrogramSetting,
    pub vectorscope: VectorscopeSetting,
    pub oscilloscope: OscilloscopeSetting,
    pub spectrum: SpectrumSetting,
    pub meters: Vec<Vec<MeterList>>,
    pub theme: Theme,
    pub current_theme_name: String,
    #[serde(skip)]
    pub theme_manager: ThemeManager,
    pub rainglow_mapping: RainglowMapping,
    #[serde(skip)]
    pub rainglow_manager: RainglowThemeManager,
}

impl Default for Setting {
    fn default() -> Self {
        let theme_manager = ThemeManager::new();
        let current_theme_name = theme_manager.current_theme_name.clone();
        let theme = theme_manager
            .get_current_theme()
            .cloned()
            .unwrap_or(DARK_THEME);
        let rainglow_mapping = RainglowMapping::default();
        let rainglow_manager = RainglowThemeManager::with_mapping(rainglow_mapping.clone());
        Self {
            audio_device: AudioDeviceSetting::default(),
            waveform: WaveformSetting::default(),
            spectrogram: SpectrogramSetting::default(),
            vectorscope: VectorscopeSetting::default(),
            oscilloscope: OscilloscopeSetting::default(),
            spectrum: SpectrumSetting::default(),
            meters: vec![
                vec![
                    MeterList::Waveform,
                    MeterList::Spectrogram,
                    MeterList::Vectorscope,
                    MeterList::Oscilloscope,
                    MeterList::Spectrum,
                    MeterList::Peak,
                    // MeterList::GPUTest,
                ],
                vec![],
            ],
            theme,
            current_theme_name,
            theme_manager,
            rainglow_mapping,
            rainglow_manager,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_persistence() {
        let mut setting = Setting::default();
        assert_eq!(setting.current_theme_name, "Dark");

        // Change theme
        setting.theme_manager.set_current_theme("Light");
        setting.current_theme_name = setting.theme_manager.current_theme_name.clone();
        assert_eq!(setting.current_theme_name, "Light");

        // Serialize and deserialize
        let serialized = serde_json::to_string(&setting).unwrap();
        let mut deserialized: Setting = serde_json::from_str(&serialized).unwrap();

        // Theme manager needs to be recreated (it's skipped in serialization)
        deserialized.theme_manager = ThemeManager::new();
        deserialized
            .theme_manager
            .set_current_theme(&deserialized.current_theme_name);

        assert_eq!(deserialized.current_theme_name, "Light");
        assert_eq!(deserialized.theme_manager.current_theme_name, "Light");
    }
}

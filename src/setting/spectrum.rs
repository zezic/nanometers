use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumSwitch {
    #[default]
    Main,
    Audio,
    Ref,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumMode {
    #[default]
    FFT,
    ColorBar,
    Both,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumChannel {
    #[default]
    LR,
    MS,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumFreqReadout {
    #[default]
    Off,
    Dyn,
    Static,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumFreqLine {
    #[default]
    Off,
    On,
    Bright,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpectrumResolution {
    FFT1024,
    #[default]
    FFT2048,
    FFT4096,
    FFT8192,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SpectrumSetting {
    #[serde(skip)]
    pub(crate) spectrum_switch: SpectrumSwitch,
    pub(crate) mode: SpectrumMode,
    pub(crate) smoothing: f32,
    pub(crate) slope: f32,
    pub(crate) channel: SpectrumChannel,
    pub(crate) low: f32,
    pub(crate) high: f32,
    pub(crate) freq_readout: SpectrumFreqReadout,
    pub(crate) freq_line: SpectrumFreqLine,
    pub(crate) ref_line: f32,
    pub(crate) threshold: f32,
    pub(crate) threshold_follow_slope: bool,
    pub(crate) update_fps: f32,
    pub(crate) resolution: SpectrumResolution,
    pub(crate) line_thickness: f32,
}

impl Default for SpectrumSetting {
    fn default() -> Self {
        Self {
            spectrum_switch: SpectrumSwitch::Main,
            mode: SpectrumMode::FFT,
            smoothing: 0.0,
            slope: 0.0,
            channel: SpectrumChannel::LR,
            low: -150.0,
            high: 20.0,
            freq_readout: SpectrumFreqReadout::Off,
            freq_line: SpectrumFreqLine::Off,
            ref_line: 0.0,
            threshold: 0.0,
            threshold_follow_slope: false,
            update_fps: 144.0,
            resolution: SpectrumResolution::FFT2048,
            line_thickness: 2.0,
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Spectrum {
    #[serde(skip)]
    pub last_rect: Option<Rect>,
    #[serde(skip)]
    pub lines: Vec<Shape>,
    #[serde(skip)]
    pub line_brightness: bool,
    #[serde(skip)]
    pub pos: Vec<f32>,
    #[serde(skip)]
    pub ch0: Vec<f32>,
    #[serde(skip)]
    pub ch1: Vec<f32>,
}

impl SpectrumResolution {
    pub fn fft_size(self) -> usize {
        match self {
            SpectrumResolution::FFT1024 => 1024,
            SpectrumResolution::FFT2048 => 2048,
            SpectrumResolution::FFT4096 => 4096,
            SpectrumResolution::FFT8192 => 8192,
        }
    }

    pub fn spectrum_bins(self) -> usize {
        self.fft_size() + 1 // Audio callback zero-pads to 2*fft_size, real FFT gives (2*fft_size)/2 + 1 = fft_size + 1 bins
    }

    pub fn bin_width(self) -> f32 {
        24000.0 / self.fft_size() as f32
    }

    pub fn generate_hann_window(self) -> Vec<f32> {
        let n = self.fft_size();
        let mut window = Vec::with_capacity(n);
        for i in 0..n {
            let value =
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos());
            window.push(value);
        }
        window
    }
}

impl Spectrum {
    pub fn new() -> Self {
        let default_resolution = SpectrumResolution::FFT2048;
        let bins = default_resolution.spectrum_bins();
        Self {
            ch0: vec![0.0; bins],
            ch1: vec![0.0; bins],
            pos: freq_to_pos_helper(default_resolution),
            ..Default::default()
        }
    }

    pub fn resize_for_resolution(&mut self, resolution: SpectrumResolution) {
        let bins = resolution.spectrum_bins();
        self.ch0.resize(bins, 0.0);
        self.ch1.resize(bins, 0.0);
        // Force position recalculation on next render
        self.last_rect = None;
    }
}

fn freq_to_pos_helper(resolution: SpectrumResolution) -> Vec<f32> {
    let spectrum_bins = resolution.spectrum_bins();
    let padded_fft_size = resolution.fft_size() * 2;
    let bin_width = 48000.0 / padded_fft_size as f32;

    // Calculate scaling coefficient so 20kHz always maps to same position (0.987627987)
    let scale_coefficient = 0.987627987 / (20000.0_f32.log10() - 1.0);

    let mut pos = Vec::with_capacity(spectrum_bins);
    for i in 0..spectrum_bins {
        if i == 0 {
            pos.push(0.0); // Will be set to rect.left() later
        } else {
            let freq = (i as f32) * bin_width;
            pos.push(scale_coefficient * (freq.log10() - 1.0)); // Normalized position
        }
    }
    pos
}

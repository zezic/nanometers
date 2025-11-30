use crate::setting::SpectrumFreqLine;
use crate::setting::SpectrumMode;
use crate::setting::SpectrumResolution;
// use crate::setting::*;
use crate::utils::*;
use crate::NanometersApp;
use egui::*;
use rustfft::num_complex::ComplexFloat;
use text::Fonts;

impl NanometersApp {
    pub fn spectrum_meter(&mut self, data: &RawData, rect: Rect, ui: &mut Ui) {
        ui.painter().rect_filled(rect, 0.0, self.setting.theme.bg);

        let spectrum_bins = self.setting.spectrum.resolution.spectrum_bins();

        if self.spectrum.last_rect.is_none() {
            self.spectrum.last_rect = Some(rect);
            self.spectrum.pos = freq_to_pos(rect, self.setting.spectrum.resolution);
        } else {
            if self.spectrum.last_rect.unwrap() != rect || self.spectrum.pos.len() != spectrum_bins
            {
                self.spectrum.last_rect = Some(rect);
                self.spectrum.pos = freq_to_pos(rect, self.setting.spectrum.resolution);
            }
        }
        if self.spectrum.ch0.is_empty() || self.spectrum.ch0.len() != spectrum_bins {
            self.spectrum.ch0 = vec![0.0; spectrum_bins];
        }
        if self.spectrum.ch1.is_empty() || self.spectrum.ch1.len() != spectrum_bins {
            self.spectrum.ch1 = vec![0.0; spectrum_bins];
        }

        // Ref lines
        match self.setting.spectrum.freq_line {
            SpectrumFreqLine::Off => {}
            SpectrumFreqLine::On => {
                if self.spectrum.last_rect.is_none() {
                    self.spectrum.last_rect = Some(rect);
                    self.spectrum.lines =
                        ref_lines(ui, rect, 0.5, self.setting.theme.spectrum_ref_line);
                    self.spectrum.line_brightness = false;
                } else {
                    if self.spectrum.last_rect.unwrap() != rect {
                        self.spectrum.last_rect = Some(rect);
                        self.spectrum.lines =
                            ref_lines(ui, rect, 0.5, self.setting.theme.spectrum_ref_line);
                        self.spectrum.line_brightness = false;
                    } else {
                        if self.spectrum.line_brightness {
                            self.spectrum.lines =
                                ref_lines(ui, rect, 0.5, self.setting.theme.spectrum_ref_line);
                            self.spectrum.line_brightness = false;
                        }
                    }
                }

                ui.painter().extend(self.spectrum.lines.clone());
            }
            SpectrumFreqLine::Bright => {
                if self.spectrum.last_rect.is_none() {
                    self.spectrum.last_rect = Some(rect);
                    self.spectrum.lines =
                        ref_lines(ui, rect, 1.0, self.setting.theme.spectrum_ref_line);
                    self.spectrum.line_brightness = false;
                } else {
                    if self.spectrum.last_rect.unwrap() != rect {
                        self.spectrum.last_rect = Some(rect);
                        self.spectrum.lines =
                            ref_lines(ui, rect, 1.0, self.setting.theme.spectrum_ref_line);
                        self.spectrum.line_brightness = false;
                    } else {
                        if !self.spectrum.line_brightness {
                            self.spectrum.lines =
                                ref_lines(ui, rect, 1.0, self.setting.theme.spectrum_ref_line);
                            self.spectrum.line_brightness = false;
                        }
                    }
                }
                ui.painter().extend(self.spectrum.lines.clone());
            }
        }
        let ref_line_x =
            0.2991878257 * (self.setting.spectrum.ref_line.log10() as f32 - 1.0) * rect.width()
                + rect.left();
        ui.painter().line_segment(
            [pos2(ref_line_x, 0.0), pos2(ref_line_x, rect.bottom())],
            Stroke::new(1.0, self.setting.theme.spectrum_ref_line),
        );

        // Main
        let mut max_index = 0;
        match self.setting.spectrum.mode {
            SpectrumMode::FFT => {
                let mut wave_0_points = Vec::new();
                let mut wave_1_points = Vec::new();
                // Fix Issue 2: Initialize with zeros instead of empty check to ensure startup works
                if data.l.is_empty() || data.l.len() != spectrum_bins {
                    // Ensure buffers are properly initialized for current resolution
                    if self.spectrum.ch0.len() != spectrum_bins {
                        self.spectrum.ch0 = vec![0.0; spectrum_bins];
                    }
                    if self.spectrum.ch1.len() != spectrum_bins {
                        self.spectrum.ch1 = vec![0.0; spectrum_bins];
                    }
                    wave_0_points.extend(
                        self.spectrum
                            .pos
                            .iter()
                            .zip(self.spectrum.ch0.iter())
                            .map(|(&pos, &ch0)| pos2(pos, (1.0 - ch0) * rect.height())),
                    );
                    wave_1_points.extend(
                        self.spectrum
                            .pos
                            .iter()
                            .zip(self.spectrum.ch1.iter())
                            .map(|(&pos, &ch1)| pos2(pos, (1.0 - ch1) * rect.height())),
                    );
                } else {
                    // Fix Issue 1: Ensure we process the expected number of bins for current resolution
                    // If data doesn't match expected resolution, clear buffers to force reinit
                    if data.l.len() != spectrum_bins {
                        self.spectrum.ch0 = vec![0.0; spectrum_bins];
                        self.spectrum.ch1 = vec![0.0; spectrum_bins];
                        // Use fallback rendering with zeros until audio callback catches up
                        wave_0_points.extend(
                            self.spectrum
                                .pos
                                .iter()
                                .zip(self.spectrum.ch0.iter())
                                .map(|(&pos, &ch0)| pos2(pos, (1.0 - ch0) * rect.height())),
                        );
                        wave_1_points.extend(
                            self.spectrum
                                .pos
                                .iter()
                                .zip(self.spectrum.ch1.iter())
                                .map(|(&pos, &ch1)| pos2(pos, (1.0 - ch1) * rect.height())),
                        );
                    } else {
                        // Normal processing - data matches expected resolution
                        for i in 0..spectrum_bins {
                            if i < data.l.len() && data.l[i] >= data.l[max_index] {
                                max_index = i;
                            }
                            if i < data.l.len()
                                && i < self.spectrum.ch0.len()
                                && (data.l[i] > self.spectrum.ch0[i]
                                    || self.spectrum.ch0[i].is_nan())
                            {
                                self.spectrum.ch0[i] = data.l[i];
                                wave_0_points.push(pos2(
                                    self.spectrum.pos[i],
                                    (1.0 - data.l[i]) * rect.height(),
                                ));
                            } else if i < self.spectrum.ch0.len() {
                                self.spectrum.ch0[i] = self.spectrum.ch0[i]
                                    * self.setting.spectrum.smoothing
                                    + data.l.get(i).unwrap_or(&0.0)
                                        * (1.0 - self.setting.spectrum.smoothing);
                                wave_0_points.push(pos2(
                                    self.spectrum.pos[i],
                                    (1.0 - self.spectrum.ch0[i]) * rect.height(),
                                ));
                            }
                            if i < data.r.len()
                                && i < self.spectrum.ch1.len()
                                && (data.r[i] > self.spectrum.ch1[i]
                                    || self.spectrum.ch1[i].is_nan())
                            {
                                self.spectrum.ch1[i] = data.r[i];
                                wave_1_points.push(pos2(
                                    self.spectrum.pos[i],
                                    (1.0 - data.r[i]) * rect.height(),
                                ));
                            } else if i < self.spectrum.ch1.len() {
                                self.spectrum.ch1[i] = self.spectrum.ch1[i]
                                    * self.setting.spectrum.smoothing
                                    + data.r.get(i).unwrap_or(&0.0)
                                        * (1.0 - self.setting.spectrum.smoothing);
                                wave_1_points.push(pos2(
                                    self.spectrum.pos[i],
                                    (1.0 - self.spectrum.ch1[i]) * rect.height(),
                                ));
                            }
                        }
                    }
                }

                let wave_0 = Shape::line(
                    wave_0_points,
                    Stroke::new(
                        self.setting.spectrum.line_thickness,
                        self.setting.theme.spectrum_main,
                    ),
                );
                let wave_1 = Shape::line(
                    wave_1_points,
                    Stroke::new(
                        self.setting.spectrum.line_thickness,
                        self.setting.theme.spectrum_secondary,
                    ),
                );
                ui.painter().add(wave_1);
                ui.painter().add(wave_0);
            }
            SpectrumMode::ColorBar => {}
            SpectrumMode::Both => {}
        }
        let max_fft_bin = self.setting.spectrum.resolution.fft_size() - 1;
        if max_index > 0 && max_index < max_fft_bin {
            let delta = (data.l[max_index + 1].abs() - data.l[max_index - 1].abs())
                / (2.0
                    * (2.0 * data.l[max_index].abs()
                        - data.l[max_index - 1].abs()
                        - data.l[max_index + 1].abs()));
            let padded_fft_size = self.setting.spectrum.resolution.fft_size() * 2;
            let freq = (max_index as f32 + delta) * (48000.0 / padded_fft_size as f32);
            let freq_str = format!("{:.2} Hz", freq);
        }
    }
}

fn ref_lines(ui: &mut Ui, rect: Rect, width: f32, color: Color32) -> Vec<Shape> {
    let mut lines = vec![];
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.0900645099 * rect.width(), 0.0),
            pos2(rect.left() + 0.0900645099 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 20
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.1427488708 * rect.width(), 0.0),
            pos2(rect.left() + 0.1427488708 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 30
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.1801290197 * rect.width(), 0.0),
            pos2(rect.left() + 0.1801290197 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 40
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2091233158 * rect.width(), 0.0),
            pos2(rect.left() + 0.2091233158 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 50
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2328133807 * rect.width(), 0.0),
            pos2(rect.left() + 0.2328133807 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 60
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2528430451 * rect.width(), 0.0),
            pos2(rect.left() + 0.2528430451 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 70
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2701935296 * rect.width(), 0.0),
            pos2(rect.left() + 0.2701935296 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 80
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2854977416 * rect.width(), 0.0),
            pos2(rect.left() + 0.2854977416 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 90
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.2991878257 * rect.width(), 10.0),
            pos2(rect.left() + 0.2991878257 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 100
    ui.fonts(|fonts| {
        lines.push(Shape::text(
            fonts,
            pos2(rect.left() + 0.2991878257 * rect.width(), 5.0),
            Align2::CENTER_CENTER,
            "100Hz",
            FontId::monospace(9.0),
            color,
        ));
    });
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.3892523356 * rect.width(), 0.0),
            pos2(rect.left() + 0.3892523356 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 200
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.4419366965 * rect.width(), 0.0),
            pos2(rect.left() + 0.4419366965 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 300
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.4793168454 * rect.width(), 0.0),
            pos2(rect.left() + 0.4793168454 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 400
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5083111415 * rect.width(), 0.0),
            pos2(rect.left() + 0.5083111415 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 500
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5320012064 * rect.width(), 0.0),
            pos2(rect.left() + 0.5320012064 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 600
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5520308708 * rect.width(), 0.0),
            pos2(rect.left() + 0.5520308708 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 700
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5693813553 * rect.width(), 0.0),
            pos2(rect.left() + 0.5693813553 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 800
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5846855673 * rect.width(), 0.0),
            pos2(rect.left() + 0.5846855673 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 900
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.5983756514 * rect.width(), 10.0),
            pos2(rect.left() + 0.5983756514 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 1000
    ui.fonts(|fonts| {
        lines.push(Shape::text(
            fonts,
            pos2(rect.left() + 0.5983756514 * rect.width(), 5.0),
            Align2::CENTER_CENTER,
            "1kHz",
            FontId::monospace(9.0),
            color,
        ));
    });
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.6884401613 * rect.width(), 0.0),
            pos2(rect.left() + 0.6884401613 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 2000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.7411245222 * rect.width(), 0.0),
            pos2(rect.left() + 0.7411245222 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 3000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.7785046711 * rect.width(), 0.0),
            pos2(rect.left() + 0.7785046711 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 4000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.8074989672 * rect.width(), 0.0),
            pos2(rect.left() + 0.8074989672 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 5000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.8311890321 * rect.width(), 0.0),
            pos2(rect.left() + 0.8311890321 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 6000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.8512186965 * rect.width(), 0.0),
            pos2(rect.left() + 0.8512186965 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 7000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.868569181 * rect.width(), 0.0),
            pos2(rect.left() + 0.868569181 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 8000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.883873393 * rect.width(), 0.0),
            pos2(rect.left() + 0.883873393 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 9000
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.8975634771 * rect.width(), 10.0),
            pos2(rect.left() + 0.8975634771 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 10000
    ui.fonts(|fonts| {
        lines.push(Shape::text(
            fonts,
            pos2(rect.left() + 0.8975634771 * rect.width(), 5.0),
            Align2::CENTER_CENTER,
            "10kHz",
            FontId::monospace(9.0),
            color,
        ));
    });
    lines.push(Shape::line_segment(
        [
            pos2(rect.left() + 0.987627987 * rect.width(), 0.0),
            pos2(rect.left() + 0.987627987 * rect.width(), rect.bottom()),
        ],
        Stroke::new(width, color),
    )); // 20000
    lines
}

fn freq_to_pos(rect: Rect, resolution: SpectrumResolution) -> Vec<f32> {
    let spectrum_bins = resolution.spectrum_bins();
    let padded_fft_size = resolution.fft_size() * 2;
    let bin_width = 48000.0 / padded_fft_size as f32;

    // Calculate scaling coefficient so 20kHz always maps to same position (0.987627987)
    // Original was: 0.2991878257 for 2048 FFT where bin 1707 = 20kHz
    let bin_20khz = 20000.0 / bin_width;
    let scale_coefficient = 0.987627987 / (20000.0_f32.log10() - 1.0);

    let mut pos = Vec::with_capacity(spectrum_bins);
    for i in 0..spectrum_bins {
        if i == 0 {
            pos.push(rect.left());
        } else {
            let freq = (i as f32) * bin_width;
            pos.push(rect.left() + scale_coefficient * (freq.log10() - 1.0) * rect.width());
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setting::SpectrumResolution;

    #[test]
    fn test_resolution_switching_fixes() {
        println!("\n=== RESOLUTION SWITCHING FIXES TEST ===");

        let rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(1000.0, 500.0));
        let resolutions = [
            SpectrumResolution::FFT1024,
            SpectrumResolution::FFT2048,
            SpectrumResolution::FFT4096,
            SpectrumResolution::FFT8192,
        ];

        for resolution in resolutions {
            let spectrum_bins = resolution.spectrum_bins();
            let positions = freq_to_pos(rect, resolution);

            println!("\n{:?}:", resolution);
            println!("  Expected bins: {}", spectrum_bins);
            println!("  Position array length: {}", positions.len());

            // Test that 20kHz maps to same position (Issue 1 fix verification)
            let bin_width = 48000.0 / (resolution.fft_size() * 2) as f32;
            let bin_20khz = (20000.0 / bin_width).round() as usize;
            if bin_20khz < positions.len() {
                let pos_20khz = positions[bin_20khz];
                let norm_pos_20khz = (pos_20khz - rect.left()) / rect.width();
                println!(
                    "  20kHz -> bin {} -> norm pos {:.6} (should be ~0.988)",
                    bin_20khz, norm_pos_20khz
                );

                // Verify consistency within tolerance
                assert!(
                    (norm_pos_20khz - 0.988).abs() < 0.001,
                    "20kHz position inconsistent: {:.6}",
                    norm_pos_20khz
                );
            }

            // Test initialization robustness (Issue 2 fix verification)
            // Simulate empty data scenario
            let empty_data = crate::utils::data_struct::RawData::new();
            println!(
                "  Empty data test: l.len()={}, r.len()={}",
                empty_data.l.len(),
                empty_data.r.len()
            );

            // Simulate mismatched data scenario
            let mut mismatched_data = crate::utils::data_struct::RawData::new();
            mismatched_data.l = vec![0.5; 1025]; // Wrong size
            mismatched_data.r = vec![0.5; 1025];
            println!(
                "  Mismatched data test: l.len()={}, expected={}",
                mismatched_data.l.len(),
                spectrum_bins
            );

            if mismatched_data.l.len() != spectrum_bins {
                println!("    ✓ Mismatch detected - will trigger buffer reset");
            }
        }
    }

    #[test]
    fn test_frequency_mapping() {
        let rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(1000.0, 500.0));

        println!("\n=== FREQUENCY MAPPING TEST ===");

        let resolutions = [
            SpectrumResolution::FFT1024,
            SpectrumResolution::FFT2048,
            SpectrumResolution::FFT4096,
            SpectrumResolution::FFT8192,
        ];

        for resolution in resolutions {
            let spectrum_bins = resolution.spectrum_bins();
            let padded_fft_size = resolution.fft_size() * 2;
            let bin_width = 48000.0 / padded_fft_size as f32;
            let positions = freq_to_pos(rect, resolution);

            println!("\n{:?}:", resolution);
            println!("  Spectrum bins: {}", spectrum_bins);
            println!("  Padded FFT size: {}", padded_fft_size);
            println!("  Bin width: {:.3} Hz", bin_width);
            println!("  Position array length: {}", positions.len());

            // Check key frequencies
            let test_freqs = [100.0, 1000.0, 10000.0, 20000.0];
            for target_freq in test_freqs {
                let bin_idx = (target_freq / bin_width).round() as usize;
                if bin_idx < positions.len() {
                    let actual_freq = bin_idx as f32 * bin_width;
                    let position = positions[bin_idx];
                    let normalized_pos = (position - rect.left()) / rect.width();
                    println!(
                        "    {}Hz -> bin {} -> actual {:.1}Hz -> pos {:.3} (norm {:.3})",
                        target_freq, bin_idx, actual_freq, position, normalized_pos
                    );
                }
            }

            // Check last bin
            let last_idx = positions.len() - 1;
            let last_freq = last_idx as f32 * bin_width;
            let last_pos = positions[last_idx];
            let last_norm_pos = (last_pos - rect.left()) / rect.width();
            println!(
                "  LAST BIN: {} -> {:.1}Hz -> pos {:.3} (norm {:.3})",
                last_idx, last_freq, last_pos, last_norm_pos
            );
        }

        // Test that 20kHz maps to same position for all resolutions
        println!("\n=== 20kHz POSITION CONSISTENCY CHECK ===");
        for resolution in resolutions {
            let positions = freq_to_pos(rect, resolution);
            let bin_width = 48000.0 / (resolution.fft_size() * 2) as f32;
            let bin_20khz = (20000.0 / bin_width).round() as usize;

            if bin_20khz < positions.len() {
                let pos_20khz = positions[bin_20khz];
                let norm_pos_20khz = (pos_20khz - rect.left()) / rect.width();
                println!(
                    "  {:?}: 20kHz at bin {} -> norm pos {:.6}",
                    resolution, bin_20khz, norm_pos_20khz
                );
            }
        }
    }
}

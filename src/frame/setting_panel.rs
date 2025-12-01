use crate::audio::*;
use crate::setting::{self, set_theme, RainglowMapping, Theme, VectorscopeMode};
use crate::utils::*;
use crate::NanometersApp;
use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    col: usize,
    row: usize,
}

#[derive(Clone, Debug)]
pub struct ThemeEditor {
    pub show_create_dialog: bool,
    pub new_theme_name: String,
    pub editing_theme: Option<String>,
    pub temp_theme: Theme,
    pub show_rename_dialog: bool,
    pub rename_old_name: String,
    pub rename_new_name: String,
    pub show_delete_dialog: bool,
    pub delete_theme_name: String,
    pub show_create_copy_dialog: bool,
    pub copy_original_name: String,
    pub copy_new_name: String,
    pub error_message: Option<String>,
    pub show_rainglow_mapping_dialog: bool,
    pub temp_rainglow_mapping: RainglowMapping,
    pub selected_rainglow_theme: String,
}

impl Default for ThemeEditor {
    fn default() -> Self {
        Self {
            show_create_dialog: false,
            new_theme_name: String::new(),
            editing_theme: None,
            temp_theme: Theme::default(),
            show_rename_dialog: false,
            rename_old_name: String::new(),
            rename_new_name: String::new(),
            show_delete_dialog: false,
            delete_theme_name: String::new(),
            show_create_copy_dialog: false,
            copy_original_name: String::new(),
            copy_new_name: String::new(),
            error_message: None,
            show_rainglow_mapping_dialog: false,
            temp_rainglow_mapping: RainglowMapping::default(),
            selected_rainglow_theme: String::new(),
        }
    }
}

impl NanometersApp {
    // Helper function to update Rainglow theme preview
    fn update_rainglow_preview(&mut self, ui: &mut egui::Ui) {
        let current_name = self.setting.theme_manager.current_theme_name.clone();

        // Only update if current theme is a Rainglow theme
        if current_name.starts_with("Rainglow: ") {
            // Update both temp and actual mapping
            self.setting.rainglow_mapping = self.theme_editor.temp_rainglow_mapping.clone();

            self.setting
                .rainglow_manager
                .set_mapping(self.setting.rainglow_mapping.clone());

            // Regenerate all Rainglow themes with new mapping
            self.setting
                .theme_manager
                .update_rainglow_themes(&self.setting.rainglow_manager);

            // Apply the updated theme immediately - same logic as regular theme switching
            if let Some(theme) = self.setting.theme_manager.get_current_theme().cloned() {
                self.setting.theme = theme.clone();
                self.setting.current_theme_name = current_name.clone();
                ui.ctx().set_visuals(set_theme(self));
                let mut audio_source_setting = self.audio_source_setting.try_lock().unwrap();
                audio_source_setting.theme = theme;
                audio_source_setting.current_theme_name = current_name;
            }
        }
    }
    pub fn waveform_setting_block(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Waveform");
                ui.horizontal(|ui| {
                    ui.label("Channel 1");
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::None,
                        "None",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::Left,
                        "Left",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::Right,
                        "Right",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::Mid,
                        "Mid",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_1,
                        setting::WaveformChannel::Side,
                        "Side",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Channel 2");
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::None,
                        "None",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::Left,
                        "Left",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::Right,
                        "Right",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::Mid,
                        "Mid",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.channel_2,
                        setting::WaveformChannel::Side,
                        "Side",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Mode");
                    ui.selectable_value(
                        &mut self.setting.waveform.mode,
                        setting::WaveformMode::Static,
                        "Static",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.mode,
                        setting::WaveformMode::MultiBand,
                        "MultiBand",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Peak History");
                    ui.selectable_value(
                        &mut self.setting.waveform.peak_history,
                        setting::WaveformHistory::Off,
                        "Off",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.peak_history,
                        setting::WaveformHistory::Fast,
                        "Fast",
                    );
                    ui.selectable_value(
                        &mut self.setting.waveform.peak_history,
                        setting::WaveformHistory::Slow,
                        "Slow",
                    );
                });
            });
        });
    }

    pub fn spectrogram_setting_block(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Spectrogram");
                ui.horizontal(|ui| {
                    ui.label("Orientation");
                    ui.selectable_value(
                        &mut self.setting.spectrogram.orientation,
                        setting::SpectrogramOrientation::H,
                        "Horizontal",
                    );
                    ui.selectable_value(
                        &mut self.setting.spectrogram.orientation,
                        setting::SpectrogramOrientation::V,
                        "Vertical",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Mode");
                    if ui
                        .selectable_value(
                            &mut self.setting.spectrogram.mode,
                            setting::SpectrogramMode::Sharp,
                            "Sharp",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.spectrogram.mode = setting::SpectrogramMode::Sharp;
                    };
                    if ui
                        .selectable_value(
                            &mut self.setting.spectrogram.mode,
                            setting::SpectrogramMode::Classic,
                            "Classic",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.spectrogram.mode = setting::SpectrogramMode::Classic;
                    };
                });
                ui.horizontal(|ui| {
                    ui.label("Curve");
                    if ui
                        .selectable_value(
                            &mut self.setting.spectrogram.curve,
                            setting::SpectrogramCurve::Linear,
                            "Linear",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.spectrogram.curve = setting::SpectrogramCurve::Linear;
                    };
                    if ui
                        .selectable_value(
                            &mut self.setting.spectrogram.curve,
                            setting::SpectrogramCurve::Logarithmic,
                            "Logarithmic",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.spectrogram.curve =
                            setting::SpectrogramCurve::Logarithmic;
                    };
                });
                ui.horizontal(|ui| {
                    ui.label("Brightness Boost");
                    if ui
                        .add(
                            egui::Slider::new(
                                &mut self.setting.spectrogram.brightness_boost,
                                0.01..=1.5,
                            )
                            .text(""),
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.spectrogram.brightness_boost =
                            self.setting.spectrogram.brightness_boost;
                    };
                });
            });
        });
    }

    pub fn oscilloscope_setting_block(&mut self, ui: &mut Ui) {
        // Oscilloscope
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Oscilloscope");
                ui.horizontal(|ui| {
                    ui.label("Follow Pitch")
                        .on_hover_text("Whether to follow the pitch");
                    if ui
                        .selectable_value(&mut self.setting.oscilloscope.follow_pitch, true, "On")
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.oscilloscope.follow_pitch = true;
                    };
                    if ui
                        .selectable_value(&mut self.setting.oscilloscope.follow_pitch, false, "Off")
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.oscilloscope.follow_pitch = false;
                    };
                });
                if self.setting.oscilloscope.follow_pitch {
                    ui.horizontal(|ui| {
                        ui.label("Cycle").on_hover_text("Display how many cycles");
                        if ui
                            .selectable_value(
                                &mut self.setting.oscilloscope.cycle,
                                setting::OscilloscopeCycle::Multi,
                                "Multi",
                            )
                            .changed()
                        {
                            let mut audio_souce_setting =
                                self.audio_source_setting.try_lock().unwrap();
                            audio_souce_setting.oscilloscope.cycle =
                                setting::OscilloscopeCycle::Multi;
                        };
                        if ui
                            .selectable_value(
                                &mut self.setting.oscilloscope.cycle,
                                setting::OscilloscopeCycle::Single,
                                "Single",
                            )
                            .changed()
                        {
                            let mut audio_souce_setting =
                                self.audio_source_setting.try_lock().unwrap();
                            audio_souce_setting.oscilloscope.cycle =
                                setting::OscilloscopeCycle::Single;
                        };
                    });
                };
                ui.horizontal(|ui| {
                    ui.label("Shadow")
                        .on_hover_text("Whether to display shadow (need more CPU)");
                    ui.selectable_value(&mut self.setting.oscilloscope.shadow, true, "On");
                    ui.selectable_value(&mut self.setting.oscilloscope.shadow, false, "Off");
                });
                ui.horizontal(|ui| {
                    ui.label("Update FPS");
                    if ui
                        .add(
                            egui::Slider::new(
                                &mut self.setting.oscilloscope.update_fps,
                                30.0..=240.0,
                            )
                            .text("fps"),
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.oscilloscope = self.setting.oscilloscope.clone();
                    }
                });
            });
        });
    }

    pub fn device_setting_block(&mut self, ui: &mut Ui) {
        // Audio Device
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Audio Device");
                if ui
                    .selectable_value(
                        &mut self.setting.audio_device.device,
                        setting::AudioDevice::OutputCapture,
                        "System Output",
                    )
                    .changed()
                {
                    self.audio_source.as_mut().unwrap().stop();
                    let callback = get_callback(
                        self.tx_data.clone().unwrap(),
                        self.audio_source_setting.clone(),
                        self.audio_source_buffer.clone(),
                    );
                    let mut system_capture = SystemCapture::new(callback);
                    system_capture.start();
                    self.audio_source = Some(Box::new(system_capture) as Box<dyn AudioSource>);
                }
                if ui
                    .selectable_value(
                        &mut self.setting.audio_device.device,
                        setting::AudioDevice::PluginCapture,
                        "Plugin Capture",
                    )
                    .changed()
                {
                    self.audio_source.as_mut().unwrap().stop();

                    let callback = get_callback(
                        self.tx_data.clone().unwrap(),
                        self.audio_source_setting.clone(),
                        self.audio_source_buffer.clone(),
                    );
                    let mut plugin_client = PluginClient::new(callback);
                    plugin_client.start();
                    self.audio_source = Some(Box::new(plugin_client) as Box<dyn AudioSource>);
                }
                if ui
                    .selectable_value(
                        &mut self.setting.audio_device.device,
                        setting::AudioDevice::InputCapture,
                        "System Input",
                    )
                    .changed()
                {
                    self.audio_source.as_mut().unwrap().stop();
                    self.audio_source = None;
                }
            });
        });
    }

    pub fn modules_sequence_block(&mut self, ui: &mut Ui) {
        // Sequence
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Modules Off/On");
                let mut from = None;
                let mut to = None;
                ui.columns(self.setting.meters.len(), |uis| {
                    for (col_idx, column) in self.setting.meters.clone().into_iter().enumerate() {
                        let ui = &mut uis[col_idx];
                        let frame = Frame::default();
                        let (_, dropped_payload) = ui.dnd_drop_zone::<Location, ()>(frame, |ui| {
                            ui.set_min_size(vec2(128.0, 100.0));
                            ui.painter().rect_filled(
                                ui.max_rect(),
                                0.0,
                                self.setting.theme.bgaccent,
                            );
                            for (row_idx, item) in column.iter().enumerate() {
                                let item_id = Id::new(("dnd", col_idx, row_idx));
                                let item_location = Location {
                                    col: col_idx,
                                    row: row_idx,
                                };
                                let response = ui
                                    .dnd_drag_source(item_id, item_location, |ui| {
                                        ui.label(item.to_string());
                                    })
                                    .response;

                                if let (Some(pointer), Some(hovered_payload)) = (
                                    ui.input(|i| i.pointer.interact_pos()),
                                    response.dnd_hover_payload::<Location>(),
                                ) {
                                    let rect = response.rect;
                                    let stroke = Stroke::new(1.0, Color32::WHITE);
                                    let insert_row_idx = if *hovered_payload == item_location {
                                        ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                                        row_idx
                                    } else if pointer.y < rect.center().y {
                                        ui.painter().hline(rect.x_range(), rect.top(), stroke);
                                        row_idx
                                    } else {
                                        ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                                        row_idx + 1
                                    };
                                    if let Some(dragged_payload) = response.dnd_release_payload() {
                                        // The user dropped onto this item.
                                        from = Some(dragged_payload);
                                        to = Some(Location {
                                            col: col_idx,
                                            row: insert_row_idx,
                                        });
                                    }
                                }
                            }
                        });
                        if let Some(dragged_payload) = dropped_payload {
                            // The user dropped onto the column, but not on any one item.
                            from = Some(dragged_payload);
                            to = Some(Location {
                                col: col_idx,
                                row: usize::MAX, // Inset last
                            });
                        }
                    }
                });
                if let (Some(from), Some(mut to)) = (from, to) {
                    if from.col == to.col {
                        // Dragging within the same column.
                        // Adjust row index if we are re-ordering:
                        to.row -= (from.row < to.row) as usize;
                    }

                    let item = self.setting.meters[from.col].remove(from.row);

                    let column = &mut self.setting.meters[to.col];
                    to.row = to.row.min(column.len());
                    column.insert(to.row, item);
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.meters = self.setting.meters.clone();
                    }
                }
            });
        });
    }

    pub fn vectorscope_settiing_block(&mut self, ui: &mut Ui) {
        // Vectorscope
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Vectorscope");
                ui.horizontal(|ui| {
                    ui.label("Mode");
                    if ui
                        .selectable_value(
                            &mut self.setting.vectorscope.mode,
                            setting::VectorscopeMode::Logarithmic,
                            "Logarithmic",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.vectorscope = self.setting.vectorscope.clone();
                    };
                    if ui
                        .selectable_value(
                            &mut self.setting.vectorscope.mode,
                            setting::VectorscopeMode::Linear,
                            "Linear",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.vectorscope = self.setting.vectorscope.clone();
                    };
                    if ui
                        .selectable_value(
                            &mut self.setting.vectorscope.mode,
                            setting::VectorscopeMode::Lissajous,
                            "Lissajous",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.vectorscope = self.setting.vectorscope.clone();
                    };
                });
                ui.horizontal(|ui| {
                    ui.label("Color");
                    if ui
                        .selectable_value(
                            &mut self.setting.vectorscope.color,
                            setting::VectorscopeColor::Static,
                            "Static",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.vectorscope = self.setting.vectorscope.clone();
                    };
                    if ui
                        .selectable_value(
                            &mut self.setting.vectorscope.color,
                            setting::VectorscopeColor::RGB,
                            "RGB",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.vectorscope = self.setting.vectorscope.clone();
                    };
                    if ui
                        .selectable_value(
                            &mut self.setting.vectorscope.color,
                            setting::VectorscopeColor::MultiBand,
                            "MultiBand",
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.vectorscope = self.setting.vectorscope.clone();
                    };
                });
                if self.setting.vectorscope.mode != VectorscopeMode::Lissajous {
                    ui.horizontal(|ui| {
                        ui.label("Polarity");
                        ui.selectable_value(
                            &mut self.setting.vectorscope.polarity,
                            setting::VectorscopePolarity::Uni,
                            "Uniploar",
                        );
                        ui.selectable_value(
                            &mut self.setting.vectorscope.polarity,
                            setting::VectorscopePolarity::Bi,
                            "Biploar",
                        );
                    });
                }
                ui.horizontal(|ui| {
                    ui.label("Normalize");
                    ui.selectable_value(&mut self.setting.vectorscope.normalize, false, "Off");
                    ui.selectable_value(&mut self.setting.vectorscope.normalize, true, "On");
                });
                if self.setting.vectorscope.mode != VectorscopeMode::Lissajous {
                    ui.horizontal(|ui| {
                        ui.label("Guides");
                        ui.selectable_value(&mut self.setting.vectorscope.guides, false, "Off");
                        ui.selectable_value(&mut self.setting.vectorscope.guides, true, "On");
                    });
                }
            });
        });
    }

    pub fn spectrum_setting_block(&mut self, ui: &mut Ui) {
        // Spectrum
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Spectrum");
                    ui.selectable_value(
                        &mut self.setting.spectrum.spectrum_switch,
                        setting::SpectrumSwitch::Main,
                        "MAIN",
                    );
                    ui.selectable_value(
                        &mut self.setting.spectrum.spectrum_switch,
                        setting::SpectrumSwitch::Audio,
                        "AUDIO",
                    );
                    ui.selectable_value(
                        &mut self.setting.spectrum.spectrum_switch,
                        setting::SpectrumSwitch::Ref,
                        "REF",
                    );
                });

                match self.setting.spectrum.spectrum_switch {
                    setting::SpectrumSwitch::Main => {
                        ui.horizontal(|ui| {
                            ui.label("Mode");
                            ui.selectable_value(
                                &mut self.setting.spectrum.mode,
                                setting::SpectrumMode::FFT,
                                "FFT",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.mode,
                                setting::SpectrumMode::ColorBar,
                                "ColorBar",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.mode,
                                setting::SpectrumMode::Both,
                                "Both",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Resolution");
                            let mut resolution_changed = false;
                            resolution_changed |= ui
                                .selectable_value(
                                    &mut self.setting.spectrum.resolution,
                                    setting::SpectrumResolution::FFT1024,
                                    "1024",
                                )
                                .changed();
                            resolution_changed |= ui
                                .selectable_value(
                                    &mut self.setting.spectrum.resolution,
                                    setting::SpectrumResolution::FFT2048,
                                    "2048",
                                )
                                .changed();
                            resolution_changed |= ui
                                .selectable_value(
                                    &mut self.setting.spectrum.resolution,
                                    setting::SpectrumResolution::FFT4096,
                                    "4096",
                                )
                                .changed();
                            resolution_changed |= ui
                                .selectable_value(
                                    &mut self.setting.spectrum.resolution,
                                    setting::SpectrumResolution::FFT8192,
                                    "8192",
                                )
                                .changed();

                            if resolution_changed {
                                // Resize spectrum buffers for new resolution
                                self.spectrum
                                    .resize_for_resolution(self.setting.spectrum.resolution);
                                // Resize audio callback sliding buffers
                                if let Ok(mut buffer) = self.audio_source_buffer.try_lock() {
                                    buffer
                                        .spectrum
                                        .resize_for_resolution(self.setting.spectrum.resolution);
                                }
                                // Propagate to audio callback
                                let mut audio_souce_setting =
                                    self.audio_source_setting.try_lock().unwrap();
                                audio_souce_setting.spectrum = self.setting.spectrum.clone();
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Smoothing");
                            ui.add(
                                egui::Slider::new(&mut self.setting.spectrum.smoothing, 0.0..=0.99)
                                    .text(""),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Slope");
                            if ui
                                .add(
                                    egui::Slider::new(&mut self.setting.spectrum.slope, -9.0..=9.0)
                                        .text("dB"),
                                )
                                .changed()
                            {
                                let mut audio_souce_setting =
                                    self.audio_source_setting.try_lock().unwrap();
                                audio_souce_setting.spectrum = self.setting.spectrum.clone();
                            };
                        });
                        ui.horizontal(|ui| {
                            ui.label("Line Thickness");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.setting.spectrum.line_thickness,
                                    0.25..=4.0,
                                )
                                .text("px"),
                            );
                        });
                    }
                    setting::SpectrumSwitch::Audio => {
                        ui.horizontal(|ui| {
                            ui.label("Channel");
                            if ui
                                .selectable_value(
                                    &mut self.setting.spectrum.channel,
                                    setting::SpectrumChannel::LR,
                                    "L/R",
                                )
                                .changed()
                            {
                                let mut audio_souce_setting =
                                    self.audio_source_setting.try_lock().unwrap();
                                audio_souce_setting.spectrum = self.setting.spectrum.clone();
                            };
                            if ui
                                .selectable_value(
                                    &mut self.setting.spectrum.channel,
                                    setting::SpectrumChannel::MS,
                                    "Mid/Side",
                                )
                                .changed()
                            {
                                let mut audio_souce_setting =
                                    self.audio_source_setting.try_lock().unwrap();
                                audio_souce_setting.spectrum = self.setting.spectrum.clone();
                            };
                        });
                        ui.horizontal(|ui| {
                            ui.label("Low");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut self.setting.spectrum.low,
                                        -150.0..=-20.0,
                                    )
                                    .text("dB"),
                                )
                                .changed()
                            {
                                let mut audio_souce_setting =
                                    self.audio_source_setting.try_lock().unwrap();
                                audio_souce_setting.spectrum = self.setting.spectrum.clone();
                            };
                        });
                        ui.horizontal(|ui| {
                            ui.label("High");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut self.setting.spectrum.high,
                                        -50.0..=20.0,
                                    )
                                    .text("dB"),
                                )
                                .changed()
                            {
                                let mut audio_souce_setting =
                                    self.audio_source_setting.try_lock().unwrap();
                                audio_souce_setting.spectrum = self.setting.spectrum.clone();
                            };
                        });
                    }
                    setting::SpectrumSwitch::Ref => {
                        ui.horizontal(|ui| {
                            ui.label("Freq Readout");
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_readout,
                                setting::SpectrumFreqReadout::Off,
                                "Off",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_readout,
                                setting::SpectrumFreqReadout::Dyn,
                                "Dyn",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_readout,
                                setting::SpectrumFreqReadout::Static,
                                "Static",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Freq Line");
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_line,
                                setting::SpectrumFreqLine::Off,
                                "Off",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_line,
                                setting::SpectrumFreqLine::On,
                                "On",
                            );
                            ui.selectable_value(
                                &mut self.setting.spectrum.freq_line,
                                setting::SpectrumFreqLine::Bright,
                                "Bright",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Ref Line");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.setting.spectrum.ref_line,
                                    0.0..=22000.0,
                                )
                                .step_by(1.0)
                                .text("Hz"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Threshold");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.setting.spectrum.threshold,
                                    -150.0..=0.0,
                                )
                                .text("dB"),
                            );
                        });
                    }
                }

                // FPS Setting for all spectrum modes
                ui.horizontal(|ui| {
                    ui.label("Update FPS");
                    if ui
                        .add(
                            egui::Slider::new(&mut self.setting.spectrum.update_fps, 30.0..=240.0)
                                .text("fps"),
                        )
                        .changed()
                    {
                        let mut audio_souce_setting = self.audio_source_setting.try_lock().unwrap();
                        audio_souce_setting.spectrum = self.setting.spectrum.clone();
                    }
                });
            });
        });
    }

    pub fn theme_setting_block(&mut self, ui: &mut Ui) {
        // Theme
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("Theme");

                // Theme selection with dropdown and navigation
                let theme_names = self.setting.theme_manager.get_theme_names();
                let current_name = self.setting.theme_manager.current_theme_name.clone();
                let mut theme_changed = false;
                let mut new_theme = None;

                ui.horizontal(|ui| {
                    // Previous theme button
                    if ui.button("◀").clicked() {
                        if let Some(current_index) =
                            theme_names.iter().position(|name| name == &current_name)
                        {
                            let prev_index = if current_index == 0 {
                                theme_names.len() - 1
                            } else {
                                current_index - 1
                            };
                            let prev_theme_name = &theme_names[prev_index];
                            if self
                                .setting
                                .theme_manager
                                .set_current_theme(prev_theme_name)
                            {
                                if let Some(theme) = self.setting.theme_manager.get_current_theme()
                                {
                                    new_theme = Some(theme.clone());
                                    theme_changed = true;
                                }
                            }
                        }
                    }

                    // Next theme button
                    if ui.button("▶").clicked() {
                        if let Some(current_index) =
                            theme_names.iter().position(|name| name == &current_name)
                        {
                            let next_index = if current_index == theme_names.len() - 1 {
                                0
                            } else {
                                current_index + 1
                            };
                            let next_theme_name = &theme_names[next_index];
                            if self
                                .setting
                                .theme_manager
                                .set_current_theme(next_theme_name)
                            {
                                if let Some(theme) = self.setting.theme_manager.get_current_theme()
                                {
                                    new_theme = Some(theme.clone());
                                    theme_changed = true;
                                }
                            }
                        }
                    }

                    // Theme dropdown
                    egui::ComboBox::from_label("Theme")
                        .selected_text(&current_name)
                        .show_ui(ui, |ui| {
                            for theme_name in &theme_names {
                                if ui
                                    .selectable_value(
                                        &mut self.setting.theme_manager.current_theme_name,
                                        theme_name.clone(),
                                        theme_name,
                                    )
                                    .clicked()
                                {
                                    if self.setting.theme_manager.set_current_theme(theme_name) {
                                        if let Some(theme) =
                                            self.setting.theme_manager.get_current_theme()
                                        {
                                            new_theme = Some(theme.clone());
                                            theme_changed = true;
                                        }
                                    }
                                }
                            }
                        });
                });

                if theme_changed {
                    if let Some(theme) = new_theme {
                        self.setting.theme = theme.clone();
                        self.setting.current_theme_name =
                            self.setting.theme_manager.current_theme_name.clone();
                        ui.ctx().set_visuals(set_theme(self));
                        let mut audio_source_setting =
                            self.audio_source_setting.try_lock().unwrap();
                        audio_source_setting.theme = theme;
                        audio_source_setting.current_theme_name =
                            self.setting.current_theme_name.clone();
                    }
                }

                // Theme management buttons
                ui.horizontal(|ui| {
                    if ui.button("New").clicked() {
                        self.theme_editor.show_create_dialog = true;
                        self.theme_editor.new_theme_name.clear();
                    }

                    if ui.button("Edit").clicked() {
                        if self.setting.theme_manager.is_builtin_theme(&current_name) {
                            // For built-in themes, show dialog to create a copy
                            self.theme_editor.show_create_copy_dialog = true;
                            self.theme_editor.copy_original_name = current_name.clone();
                            self.theme_editor.copy_new_name = format!("{} Copy", current_name);
                        } else {
                            // For custom themes, edit directly
                            self.theme_editor.editing_theme = Some(current_name.clone());
                            if let Some(theme) = self.setting.theme_manager.get_current_theme() {
                                self.theme_editor.temp_theme = theme.clone();
                            }
                        }
                    }

                    if !self.setting.theme_manager.is_builtin_theme(&current_name) {
                        if ui.button("Rename").clicked() {
                            self.theme_editor.show_rename_dialog = true;
                            self.theme_editor.rename_old_name = current_name.clone();
                            self.theme_editor.rename_new_name = current_name.clone();
                        }

                        if ui.button("Delete").clicked() {
                            self.theme_editor.show_delete_dialog = true;
                            self.theme_editor.delete_theme_name = current_name;
                        }
                    }

                    if ui.button("Rainglow Mapping").clicked() {
                        self.theme_editor.show_rainglow_mapping_dialog = true;
                        self.theme_editor.temp_rainglow_mapping =
                            self.setting.rainglow_mapping.clone();
                        // Set first rainglow theme as selected if none selected
                        if self.theme_editor.selected_rainglow_theme.is_empty() {
                            let rainglow_names = self.setting.rainglow_manager.get_theme_names();
                            if !rainglow_names.is_empty() {
                                self.theme_editor.selected_rainglow_theme =
                                    rainglow_names[0].clone();
                            }
                        }
                    }
                });

                // Theme editor dialogs
                self.theme_editor_dialogs(ui);
            });
        });
    }

    fn theme_editor_dialogs(&mut self, ui: &mut Ui) {
        // Create new theme dialog
        if self.theme_editor.show_create_dialog {
            let mut create_clicked = false;
            let mut cancel_clicked = false;
            let theme_name = self.theme_editor.new_theme_name.clone();

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Create New Theme");
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.theme_editor.new_theme_name);
                    });
                    ui.horizontal(|ui| {
                        create_clicked = ui.button("Create").clicked();
                        cancel_clicked = ui.button("Cancel").clicked();
                    });
                });
            });

            if create_clicked && !theme_name.is_empty() {
                match self.setting.theme_manager.create_new_theme(&theme_name) {
                    Ok(_) => {
                        self.theme_editor.show_create_dialog = false;
                        self.theme_editor.new_theme_name.clear();
                    }
                    Err(e) => {
                        self.theme_editor.error_message =
                            Some(format!("Error creating theme: {}", e));
                    }
                }
            } else if cancel_clicked {
                self.theme_editor.show_create_dialog = false;
                self.theme_editor.new_theme_name.clear();
            }
        }

        // Create copy dialog for built-in themes
        if self.theme_editor.show_create_copy_dialog {
            let mut create_clicked = false;
            let mut cancel_clicked = false;
            let original_name = self.theme_editor.copy_original_name.clone();
            let new_name = self.theme_editor.copy_new_name.clone();

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Create Copy of Built-in Theme");
                    ui.label(format!(
                        "Built-in theme '{}' cannot be edited directly.",
                        original_name
                    ));
                    ui.label("Create a copy to customize:");
                    ui.horizontal(|ui| {
                        ui.label("Copy name:");
                        ui.text_edit_singleline(&mut self.theme_editor.copy_new_name);
                    });
                    ui.horizontal(|ui| {
                        create_clicked = ui.button("Create Copy").clicked();
                        cancel_clicked = ui.button("Cancel").clicked();
                    });
                });
            });

            if create_clicked && !new_name.is_empty() {
                match self
                    .setting
                    .theme_manager
                    .save_theme_as_copy(&original_name, &new_name)
                {
                    Ok(new_theme) => {
                        // Switch to the new theme and start editing it
                        self.setting.theme_manager.set_current_theme(&new_name);
                        self.setting.theme = new_theme.clone();
                        self.setting.current_theme_name = new_name.clone();
                        ui.ctx().set_visuals(set_theme(self));
                        let mut audio_source_setting =
                            self.audio_source_setting.try_lock().unwrap();
                        audio_source_setting.theme = new_theme.clone();
                        audio_source_setting.current_theme_name = new_name.clone();

                        // Start editing the new copy
                        self.theme_editor.editing_theme = Some(new_name);
                        self.theme_editor.temp_theme = new_theme;
                        self.theme_editor.show_create_copy_dialog = false;
                    }
                    Err(e) => {
                        self.theme_editor.error_message =
                            Some(format!("Error creating copy: {}", e));
                    }
                }
            } else if cancel_clicked {
                self.theme_editor.show_create_copy_dialog = false;
            }
        }

        // Edit theme dialog
        if let Some(ref editing_name) = self.theme_editor.editing_theme.clone() {
            let mut save_clicked = false;
            let mut cancel_clicked = false;

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading(format!("Edit Theme: {}", editing_name));

                    // Color editors
                    ui.horizontal(|ui| {
                        ui.label("Main:");
                        ui.color_edit_button_srgba(&mut self.theme_editor.temp_theme.main);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Background:");
                        ui.color_edit_button_srgba(&mut self.theme_editor.temp_theme.bg);
                    });
                    ui.horizontal(|ui| {
                        ui.label("BG Accent:");
                        ui.color_edit_button_srgba(&mut self.theme_editor.temp_theme.bgaccent);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Text:");
                        ui.color_edit_button_srgba(&mut self.theme_editor.temp_theme.text);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Accent:");
                        ui.color_edit_button_srgba(&mut self.theme_editor.temp_theme.accent);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Frame:");
                        ui.color_edit_button_srgba(&mut self.theme_editor.temp_theme.frame);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Selection:");
                        ui.color_edit_button_srgba(&mut self.theme_editor.temp_theme.selection);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Spectrum Main:");
                        ui.color_edit_button_srgba(&mut self.theme_editor.temp_theme.spectrum_main);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Spectrum Secondary:");
                        ui.color_edit_button_srgba(
                            &mut self.theme_editor.temp_theme.spectrum_secondary,
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Spectrum Ref Line:");
                        ui.color_edit_button_srgba(
                            &mut self.theme_editor.temp_theme.spectrum_ref_line,
                        );
                    });

                    ui.horizontal(|ui| {
                        save_clicked = ui.button("Save").clicked();
                        cancel_clicked = ui.button("Cancel").clicked();
                    });
                });
            });

            if save_clicked {
                let temp_theme = self.theme_editor.temp_theme.clone();
                match self.setting.theme_manager.save_theme(temp_theme.clone()) {
                    Ok(_) => {
                        self.setting.theme = temp_theme.clone();
                        ui.ctx().set_visuals(set_theme(self));
                        let mut audio_source_setting =
                            self.audio_source_setting.try_lock().unwrap();
                        audio_source_setting.theme = temp_theme;
                        self.theme_editor.editing_theme = None;
                    }
                    Err(e) => {
                        self.theme_editor.error_message =
                            Some(format!("Error saving theme: {}", e));
                    }
                }
            } else if cancel_clicked {
                self.theme_editor.editing_theme = None;
            }
        }

        // Rename theme dialog
        if self.theme_editor.show_rename_dialog {
            let mut rename_clicked = false;
            let mut cancel_clicked = false;
            let old_name = self.theme_editor.rename_old_name.clone();
            let new_name = self.theme_editor.rename_new_name.clone();

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Rename Theme");
                    ui.horizontal(|ui| {
                        ui.label("New name:");
                        ui.text_edit_singleline(&mut self.theme_editor.rename_new_name);
                    });
                    ui.horizontal(|ui| {
                        rename_clicked = ui.button("Rename").clicked();
                        cancel_clicked = ui.button("Cancel").clicked();
                    });
                });
            });

            if rename_clicked && !new_name.is_empty() {
                match self
                    .setting
                    .theme_manager
                    .rename_theme(&old_name, &new_name)
                {
                    Ok(_) => {
                        self.theme_editor.show_rename_dialog = false;
                    }
                    Err(e) => {
                        self.theme_editor.error_message =
                            Some(format!("Error renaming theme: {}", e));
                    }
                }
            } else if cancel_clicked {
                self.theme_editor.show_rename_dialog = false;
            }
        }

        // Delete confirmation dialog
        if self.theme_editor.show_delete_dialog {
            let mut delete_clicked = false;
            let mut cancel_clicked = false;
            let delete_name = self.theme_editor.delete_theme_name.clone();

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Delete Theme");
                    ui.label(format!(
                        "Are you sure you want to delete '{}'?",
                        delete_name
                    ));
                    ui.horizontal(|ui| {
                        delete_clicked = ui.button("Delete").clicked();
                        cancel_clicked = ui.button("Cancel").clicked();
                    });
                });
            });

            if delete_clicked {
                match self.setting.theme_manager.delete_theme(&delete_name) {
                    Ok(_) => {
                        // Switch to current theme after deletion
                        if let Some(theme) = self.setting.theme_manager.get_current_theme() {
                            let new_theme = theme.clone();
                            self.setting.theme = new_theme.clone();
                            self.setting.current_theme_name =
                                self.setting.theme_manager.current_theme_name.clone();
                            ui.ctx().set_visuals(set_theme(self));
                            let mut audio_source_setting =
                                self.audio_source_setting.try_lock().unwrap();
                            audio_source_setting.theme = new_theme;
                            audio_source_setting.current_theme_name =
                                self.setting.current_theme_name.clone();
                        }
                        self.theme_editor.show_delete_dialog = false;
                    }
                    Err(e) => {
                        self.theme_editor.error_message =
                            Some(format!("Error deleting theme: {}", e));
                    }
                }
            } else if cancel_clicked {
                self.theme_editor.show_delete_dialog = false;
            }
        }

        // Rainglow mapping dialog
        if self.theme_editor.show_rainglow_mapping_dialog {
            let mut save_clicked = false;
            let mut cancel_clicked = false;

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Rainglow Color Mapping");

                    // Theme selector for preview
                    ui.horizontal(|ui| {
                        ui.label("Preview Theme:");
                        let theme_names = self.setting.rainglow_manager.get_theme_names();
                        if egui::ComboBox::from_label("")
                            .selected_text(&self.theme_editor.selected_rainglow_theme)
                            .show_ui(ui, |ui| {
                                for theme_name in &theme_names {
                                    ui.selectable_value(
                                        &mut self.theme_editor.selected_rainglow_theme,
                                        theme_name.clone(),
                                        theme_name,
                                    );
                                }
                            })
                            .response
                            .changed()
                        {
                            // Switch to selected Rainglow theme for preview
                            let rainglow_theme_name =
                                format!("Rainglow: {}", self.theme_editor.selected_rainglow_theme);
                            self.setting
                                .theme_manager
                                .set_current_theme(&rainglow_theme_name);
                            self.update_rainglow_preview(ui);
                        }
                    });

                    ui.separator();

                    // Get available color keys from selected theme
                    let available_keys = if !self.theme_editor.selected_rainglow_theme.is_empty() {
                        self.setting
                            .rainglow_manager
                            .get_available_color_keys(&self.theme_editor.selected_rainglow_theme)
                    } else {
                        Vec::new()
                    };

                    // Color mapping controls
                    ui.columns(2, |columns| {
                        columns[0].vertical(|ui| {
                            ui.heading("App Colors");

                            ui.horizontal(|ui| {
                                ui.label("Main:");
                                let old_value =
                                    self.theme_editor.temp_rainglow_mapping.main.clone();
                                egui::ComboBox::from_id_source("main_mapping")
                                    .selected_text(&self.theme_editor.temp_rainglow_mapping.main)
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self.theme_editor.temp_rainglow_mapping.main,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value != self.theme_editor.temp_rainglow_mapping.main {
                                    self.update_rainglow_preview(ui);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Background:");
                                let old_value = self.theme_editor.temp_rainglow_mapping.bg.clone();
                                egui::ComboBox::from_id_source("bg_mapping")
                                    .selected_text(&self.theme_editor.temp_rainglow_mapping.bg)
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self.theme_editor.temp_rainglow_mapping.bg,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value != self.theme_editor.temp_rainglow_mapping.bg {
                                    self.update_rainglow_preview(ui);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("BG Accent:");
                                let old_value =
                                    self.theme_editor.temp_rainglow_mapping.bgaccent.clone();
                                egui::ComboBox::from_id_source("bgaccent_mapping")
                                    .selected_text(
                                        &self.theme_editor.temp_rainglow_mapping.bgaccent,
                                    )
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self
                                                    .theme_editor
                                                    .temp_rainglow_mapping
                                                    .bgaccent,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value != self.theme_editor.temp_rainglow_mapping.bgaccent {
                                    self.update_rainglow_preview(ui);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Text:");
                                let old_value =
                                    self.theme_editor.temp_rainglow_mapping.text.clone();
                                egui::ComboBox::from_id_source("text_mapping")
                                    .selected_text(&self.theme_editor.temp_rainglow_mapping.text)
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self.theme_editor.temp_rainglow_mapping.text,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value != self.theme_editor.temp_rainglow_mapping.text {
                                    self.update_rainglow_preview(ui);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Accent:");
                                let old_value =
                                    self.theme_editor.temp_rainglow_mapping.accent.clone();
                                egui::ComboBox::from_id_source("accent_mapping")
                                    .selected_text(&self.theme_editor.temp_rainglow_mapping.accent)
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self.theme_editor.temp_rainglow_mapping.accent,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value != self.theme_editor.temp_rainglow_mapping.accent {
                                    self.update_rainglow_preview(ui);
                                }
                            });
                        });

                        columns[1].vertical(|ui| {
                            ui.heading("Spectrum Colors");

                            ui.horizontal(|ui| {
                                ui.label("Frame:");
                                let old_value =
                                    self.theme_editor.temp_rainglow_mapping.frame.clone();
                                egui::ComboBox::from_id_source("frame_mapping")
                                    .selected_text(&self.theme_editor.temp_rainglow_mapping.frame)
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self.theme_editor.temp_rainglow_mapping.frame,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value != self.theme_editor.temp_rainglow_mapping.frame {
                                    self.update_rainglow_preview(ui);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Selection:");
                                let old_value =
                                    self.theme_editor.temp_rainglow_mapping.selection.clone();
                                egui::ComboBox::from_id_source("selection_mapping")
                                    .selected_text(
                                        &self.theme_editor.temp_rainglow_mapping.selection,
                                    )
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self
                                                    .theme_editor
                                                    .temp_rainglow_mapping
                                                    .selection,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value != self.theme_editor.temp_rainglow_mapping.selection {
                                    self.update_rainglow_preview(ui);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Spectrum Main:");
                                let old_value = self
                                    .theme_editor
                                    .temp_rainglow_mapping
                                    .spectrum_main
                                    .clone();
                                egui::ComboBox::from_id_source("spectrum_main_mapping")
                                    .selected_text(
                                        &self.theme_editor.temp_rainglow_mapping.spectrum_main,
                                    )
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self
                                                    .theme_editor
                                                    .temp_rainglow_mapping
                                                    .spectrum_main,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value
                                    != self.theme_editor.temp_rainglow_mapping.spectrum_main
                                {
                                    self.update_rainglow_preview(ui);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Spectrum Secondary:");
                                let old_value = self
                                    .theme_editor
                                    .temp_rainglow_mapping
                                    .spectrum_secondary
                                    .clone();
                                egui::ComboBox::from_id_source("spectrum_secondary_mapping")
                                    .selected_text(
                                        &self.theme_editor.temp_rainglow_mapping.spectrum_secondary,
                                    )
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self
                                                    .theme_editor
                                                    .temp_rainglow_mapping
                                                    .spectrum_secondary,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value
                                    != self.theme_editor.temp_rainglow_mapping.spectrum_secondary
                                {
                                    self.update_rainglow_preview(ui);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Reference Line:");
                                let old_value = self
                                    .theme_editor
                                    .temp_rainglow_mapping
                                    .spectrum_ref_line
                                    .clone();
                                egui::ComboBox::from_id_source("spectrum_ref_mapping")
                                    .selected_text(
                                        &self.theme_editor.temp_rainglow_mapping.spectrum_ref_line,
                                    )
                                    .show_ui(ui, |ui| {
                                        for key in &available_keys {
                                            ui.selectable_value(
                                                &mut self
                                                    .theme_editor
                                                    .temp_rainglow_mapping
                                                    .spectrum_ref_line,
                                                key.clone(),
                                                key,
                                            );
                                        }
                                    });
                                if old_value
                                    != self.theme_editor.temp_rainglow_mapping.spectrum_ref_line
                                {
                                    self.update_rainglow_preview(ui);
                                }
                            });
                        });
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        save_clicked = ui.button("Save Mapping").clicked();
                        cancel_clicked = ui.button("Cancel").clicked();
                    });
                });
            });

            if save_clicked {
                // Save the mapping and update themes
                self.setting.rainglow_mapping = self.theme_editor.temp_rainglow_mapping.clone();
                self.setting
                    .rainglow_manager
                    .set_mapping(self.setting.rainglow_mapping.clone());
                self.setting
                    .theme_manager
                    .update_rainglow_themes(&self.setting.rainglow_manager);
                self.theme_editor.show_rainglow_mapping_dialog = false;
            } else if cancel_clicked {
                self.theme_editor.show_rainglow_mapping_dialog = false;
            }
        }

        // Show error message if any
        if let Some(ref error) = self.theme_editor.error_message.clone() {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.colored_label(Color32::RED, error);
                    if ui.button("OK").clicked() {
                        self.theme_editor.error_message = None;
                    }
                });
            });
        }
    }

    pub fn cpu_setting_block(&mut self, ui: &mut Ui) {
        // CPU
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("CPU");
                ui.horizontal(|ui| {
                    ui.label("FPS");
                    ui.label(format!("{:.1}", self.frame_history.fps()));
                });
                ui.horizontal(|ui| {
                    ui.label("Mean Frame Time");
                    ui.label(format!(
                        "{:.1} ms",
                        self.frame_history.mean_frame_time() * 1000.0
                    ));
                });
            });
        });
    }
}

use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OscilloscopeCycle {
    #[default]
    Multi,
    Single,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OscilloscopeSetting {
    pub(crate) follow_pitch: bool,
    pub(crate) cycle: OscilloscopeCycle,
    pub(crate) shadow: bool,
    pub(crate) update_fps: f32,
}

impl Default for OscilloscopeSetting {
    fn default() -> Self {
        Self {
            follow_pitch: false,
            cycle: OscilloscopeCycle::Multi,
            shadow: false,
            update_fps: 144.0,
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Oscilloscope {
    #[serde(skip)]
    pub(crate) plot: Vec<Shape>,
}

#[derive(Debug, Clone, Default)]
pub struct OscCalcBuffer {
    pub even_trun: bool,
    pub first_turn: bool,
    pub raw: Vec<f32>,
    pub last: Option<f32>,
}

impl OscCalcBuffer {
    pub fn new() -> Self {
        Self {
            even_trun: false,
            first_turn: false,
            raw: Vec::new(),
            last: None,
        }
    }
    pub fn clear(&mut self) {
        self.even_trun = false;
        self.first_turn = false;
        self.raw.clear();
        self.last = None;
    }
}

#[derive(Debug, Clone, Default)]
pub struct OscilloscopeSendData {
    pub len: usize,
    pub data: Vec<f32>,
}

impl OscilloscopeSendData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

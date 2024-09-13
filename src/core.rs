//! ProjectM for Rust
//!
//! This library contains bindings to libprojectm. Its purpose
//! is to read an audio input and to produce mesmerizing visuals
//! by detecting tempo, and rendering advanced equations into a
//! limitless array of user-contributed visualizations.
//!
//! # Example
//!
// ! use projectm_rs::core::*;
// !
// ! let ProjectMHandle = inner::create();
//!

mod inner;

pub struct ProjectMHandle(pub(crate) *mut inner::projectm);
// SAFETY: since the pointer is a private field, the only way to get access to it is through the struct
unsafe impl Send for ProjectMHandle {}
// SAFETY: this is probably fine? because of the mutable reference requirements.
unsafe impl Sync for ProjectMHandle {}

impl Drop for ProjectMHandle {
    fn drop(&mut self) {
        // SAFETY: this handle is going out of scope, and won't be called again
        unsafe { inner::destroy(self) }
    }
}

pub type ProjectMChannels = u32;
pub const MONO: ProjectMChannels = 1;
pub const STEREO: ProjectMChannels = 2;

pub type ProjectMTouchType = u32;
pub const TOUCH_TYPE_RANDOM: ProjectMTouchType = 0;
pub const TOUCH_TYPE_CIRCLE: ProjectMTouchType = 1;
pub const TOUCH_TYPE_RADIAL_BLOB: ProjectMTouchType = 2;
pub const TOUCH_TYPE_BLOB2: ProjectMTouchType = 3;
pub const TOUCH_TYPE_BLOB3: ProjectMTouchType = 4;
pub const TOUCH_TYPE_DERIVATIVE_LINE: ProjectMTouchType = 5;
pub const TOUCH_TYPE_BLOB5: ProjectMTouchType = 6;
pub const TOUCH_TYPE_LINE: ProjectMTouchType = 7;
pub const TOUCH_TYPE_DOUBLE_LINE: ProjectMTouchType = 8;

#[derive(PartialEq, Eq, Debug)]
pub enum TouchType {
    Random,
    Circle,
    RadialBlob,
    Blob2,
    Blob3,
    Blob5,
    Line,
    DoubleLine,
    DerivitaveLine,
}

impl From<TouchType> for ProjectMTouchType {
    fn from(val: TouchType) -> Self {
        match val {
            TouchType::Random => TOUCH_TYPE_RANDOM,
            TouchType::Circle => TOUCH_TYPE_CIRCLE,
            TouchType::RadialBlob => TOUCH_TYPE_RADIAL_BLOB,
            TouchType::Blob2 => TOUCH_TYPE_BLOB2,
            TouchType::Blob3 => TOUCH_TYPE_BLOB3,
            TouchType::Blob5 => TOUCH_TYPE_BLOB5,
            TouchType::Line => TOUCH_TYPE_LINE,
            TouchType::DoubleLine => TOUCH_TYPE_DOUBLE_LINE,
            TouchType::DerivitaveLine => TOUCH_TYPE_DERIVATIVE_LINE,
        }
    }
}

pub struct ProjectM {
    instance: ProjectMHandle,
}

impl Default for ProjectM {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectM {
    pub fn new() -> Self {
        let instance = ProjectMHandle(inner::create());

        ProjectM { instance }
    }

    pub fn instance(&self) -> &ProjectMHandle {
        &self.instance
    }

    pub fn instance_mut(&mut self) -> &mut ProjectMHandle {
        &mut self.instance
    }

    pub fn load_preset_file(&mut self, filename: &str, smooth_transition: bool) {
        inner::load_preset_file(&mut self.instance, filename, smooth_transition);
    }

    pub fn load_preset_data(&mut self, data: &str, smooth_transition: bool) {
        inner::load_preset_data(&mut self.instance, data, smooth_transition);
    }

    pub fn reset_textures(&mut self) {
        inner::reset_textures(&mut self.instance);
    }

    pub fn get_version_components() -> (i32, i32, i32) {
        inner::get_version_components()
    }

    pub fn get_version_string() -> String {
        inner::get_version_string()
    }

    pub fn get_vcs_version_string() -> String {
        inner::get_vcs_version_string()
    }

    pub fn set_preset_switch_requested_event_callback<F: FnMut(bool) + 'static>(
        &mut self,
        callback: F,
    ) {
        inner::set_preset_switch_requested_event_callback(&mut self.instance, callback);
    }

    pub fn set_preset_switch_failed_event_callback<F: FnMut(String, String) + 'static>(
        &mut self,
        callback: F,
    ) {
        inner::set_preset_switch_failed_event_callback(&mut self.instance, callback);
    }

    pub fn set_texture_search_paths(&mut self, texture_search_paths: &[String], count: usize) {
        inner::set_texture_search_paths(&mut self.instance, texture_search_paths, count);
    }

    pub fn get_beat_sensitivity(&self) -> f32 {
        inner::get_beat_sensitivity(&self.instance)
    }

    pub fn set_beat_sensitivity(&mut self, sensitivity: f32) {
        inner::set_beat_sensitivity(&mut self.instance, sensitivity);
    }

    pub fn get_hard_cut_duration(&self) -> f64 {
        inner::get_hard_cut_duration(&self.instance)
    }

    pub fn set_hard_cut_duration(&mut self, seconds: f64) {
        inner::set_hard_cut_duration(&mut self.instance, seconds);
    }

    pub fn get_hard_cut_enabled(&self) -> bool {
        inner::get_hard_cut_enabled(&self.instance)
    }

    pub fn set_hard_cut_enabled(&mut self, enabled: bool) {
        inner::set_hard_cut_enabled(&mut self.instance, enabled);
    }

    pub fn get_hard_cut_sensitivity(&self) -> f32 {
        inner::get_hard_cut_sensitivity(&self.instance)
    }

    pub fn set_hard_cut_sensitivity(&mut self, sensitivity: f32) {
        inner::set_hard_cut_sensitivity(&mut self.instance, sensitivity);
    }

    pub fn get_soft_cut_duration(&self) -> f64 {
        inner::get_soft_cut_duration(&self.instance)
    }

    pub fn set_soft_cut_duration(&mut self, seconds: f64) {
        inner::set_soft_cut_duration(&mut self.instance, seconds);
    }

    pub fn get_preset_duration(&self) -> f64 {
        inner::get_preset_duration(&self.instance)
    }

    pub fn set_preset_duration(&mut self, seconds: f64) {
        inner::set_preset_duration(&mut self.instance, seconds);
    }

    pub fn get_mesh_size(&self) -> (usize, usize) {
        inner::get_mesh_size(&self.instance)
    }

    pub fn set_mesh_size(&mut self, mesh_x: usize, mesh_y: usize) {
        inner::set_mesh_size(&mut self.instance, mesh_x, mesh_y);
    }

    pub fn get_fps(&self) -> u32 {
        inner::get_fps(&self.instance)
    }

    pub fn set_fps(&mut self, fps: u32) {
        inner::set_fps(&mut self.instance, fps);
    }

    pub fn get_aspect_correction(&self) -> bool {
        inner::get_aspect_correction(&self.instance)
    }

    pub fn set_aspect_correction(&mut self, enabled: bool) {
        inner::set_aspect_correction(&mut self.instance, enabled);
    }

    pub fn get_easter_egg(&self) -> f32 {
        inner::get_easter_egg(&self.instance)
    }

    pub fn set_easter_egg(&mut self, sensitivity: f32) {
        inner::set_easter_egg(&mut self.instance, sensitivity);
    }

    pub fn get_preset_locked(&self) -> bool {
        inner::get_preset_locked(&self.instance)
    }

    pub fn set_preset_locked(&mut self, lock: bool) {
        inner::set_preset_locked(&mut self.instance, lock);
    }

    pub fn get_window_size(&self) -> (usize, usize) {
        inner::get_window_size(&self.instance)
    }

    pub fn set_window_size(&mut self, width: usize, height: usize) {
        inner::set_window_size(&mut self.instance, width, height);
    }

    pub fn render_frame(&mut self) {
        inner::render_frame(&mut self.instance);
    }

    pub fn touch(&mut self, x: f32, y: f32, pressure: i32, touch_type: TouchType) {
        inner::touch(&mut self.instance, x, y, pressure, touch_type.into());
    }

    pub fn touch_drag(&mut self, x: f32, y: f32, pressure: i32) {
        inner::touch_drag(&mut self.instance, x, y, pressure);
    }

    pub fn touch_destroy(&mut self, x: f32, y: f32) {
        inner::touch_destroy(&mut self.instance, x, y);
    }

    pub fn touch_destroy_all(&mut self) {
        inner::touch_destroy_all(&mut self.instance);
    }

    pub fn pcm_get_max_samples() -> u32 {
        inner::pcm_get_max_samples()
    }

    pub fn pcm_add_float(&mut self, samples: Vec<f32>, channels: ProjectMChannels) {
        inner::pcm_add_float(&mut self.instance, samples, channels);
    }

    pub fn pcm_add_int16(&mut self, samples: Vec<i16>, channels: ProjectMChannels) {
        inner::pcm_add_int16(&mut self.instance, samples, channels);
    }

    pub fn pcm_add_uint8(&mut self, samples: Vec<u8>, channels: ProjectMChannels) {
        inner::pcm_add_uint8(&mut self.instance, samples, channels);
    }

    pub fn write_debug_image_on_next_frame(&self, output_file: Option<&String>) {
        inner::write_debug_image_on_next_frame(&self.instance, output_file);
    }
}

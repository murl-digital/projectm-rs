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

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    time::Duration,
};

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
    /// Creates a new [`ProjectM`] instance.
    pub fn new() -> Self {
        let instance = ProjectMHandle(inner::create());

        ProjectM { instance }
    }

    /// Returns a reference to the inner [`ProjectMHandle`]
    pub fn instance(&self) -> &ProjectMHandle {
        &self.instance
    }

    /// Returns a reference to the inner [`ProjectMHandle`]
    pub fn instance_mut(&mut self) -> &mut ProjectMHandle {
        &mut self.instance
    }

    /// Loads a new preset from a file on disk. If loading fails for any reason, ProjectM will stay on the currently loaded preset.
    ///
    /// # Arguments
    /// - `file`: The path to the file on disk
    /// - `smooth_transition`: Whether or not ProjectM blends between the current preset and the next
    pub fn load_preset_file(&mut self, file: impl AsRef<Path>, smooth_transition: bool) {
        inner::load_preset_file(
            &mut self.instance,
            file.as_ref().as_os_str(),
            smooth_transition,
        )
    }

    /// Loads the default idle preset (The one with the ProjectM logo and headphones. Yeah, that one).
    ///
    /// # Arguments
    /// - `smooth_transition`: Whether or not ProjectM blends between the current preset and the next
    pub fn load_idle_preset(&mut self, smooth_transition: bool) {
        inner::load_preset_file(&mut self.instance, OsStr::new("idle://"), smooth_transition)
    }

    /// Loads a preset from a string directly. It's assumed that the preset is in the Milkdrop format
    ///
    /// # Arguments
    /// - `data`: The preset data.
    /// - `smooth_transition`: Whether or not ProjectM blends between the current preset and the next
    pub fn load_preset_data(&mut self, data: &str, smooth_transition: bool) {
        inner::load_preset_data(&mut self.instance, data, smooth_transition);
    }

    /// Reloads all textures.
    /// Calling this method will clear and reload all textures, including the main rendering texture.
    ///
    /// Can cause a small delay/lag in rendering. Only use if texture paths were changed.
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

    /// Sets the callback function used when ProjectM wants to switch to a new preset.
    /// Trying to acquire a lock on the [`ProjectM`] instance in the callback will cause a deadlock!
    ///
    /// # Arguments
    /// - `callback`: The callback. The boolean parameter is whether or not this is a hard cut.
    pub fn set_preset_switch_requested_event_callback<F: FnMut(bool) + 'static>(
        &mut self,
        callback: F,
    ) {
        inner::set_preset_switch_requested_event_callback(&mut self.instance, callback);
    }

    /// Sets the callback function when switching to a new preset fails.
    /// Trying to acquire a lock on the [`ProjectM`] instance in the callback will cause a deadlock!
    ///
    /// # Arguments
    /// - `callback`: The callback. The first string is the filename of the preset that attempted to load, and the second is the error message.
    pub fn set_preset_switch_failed_event_callback<F: FnMut(String, String) + 'static>(
        &mut self,
        callback: F,
    ) {
        inner::set_preset_switch_failed_event_callback(&mut self.instance, callback);
    }

    /// Sets the texture search paths. Calling this will clear and reload all textures and cause some lag, similar to [`ProjectM::reset_textures`]
    pub fn set_texture_search_paths(&mut self, texture_search_paths: &[PathBuf], count: usize) {
        inner::set_texture_search_paths(&mut self.instance, texture_search_paths, count);
    }

    /// Returns the beat sensitivity of this [`ProjectM`].
    pub fn get_beat_sensitivity(&self) -> f32 {
        inner::get_beat_sensitivity(&self.instance)
    }

    /// Sets the beat sensitivity of this [`ProjectM`].
    pub fn set_beat_sensitivity(&mut self, sensitivity: f32) {
        inner::set_beat_sensitivity(&mut self.instance, sensitivity);
    }

    /// Returns the hard cut duration of this [`ProjectM`].
    pub fn get_hard_cut_duration(&self) -> f64 {
        inner::get_hard_cut_duration(&self.instance)
    }

    /// Sets the hard cut duration of this [`ProjectM`].
    pub fn set_hard_cut_duration(&mut self, seconds: f64) {
        inner::set_hard_cut_duration(&mut self.instance, seconds);
    }

    /// Returns whether or not hard cuts are enabled for this [`ProjectM`].
    pub fn get_hard_cut_enabled(&self) -> bool {
        inner::get_hard_cut_enabled(&self.instance)
    }

    /// Sets whether or not hard cuts are enabled for this [`ProjectM`].
    pub fn set_hard_cut_enabled(&mut self, enabled: bool) {
        inner::set_hard_cut_enabled(&mut self.instance, enabled);
    }

    /// Returns the hard cut sensitivity of this [`ProjectM`].
    pub fn get_hard_cut_sensitivity(&self) -> f32 {
        inner::get_hard_cut_sensitivity(&self.instance)
    }

    /// Sets the hard cut sensitivity of this [`ProjectM`].
    pub fn set_hard_cut_sensitivity(&mut self, sensitivity: f32) {
        inner::set_hard_cut_sensitivity(&mut self.instance, sensitivity);
    }

    /// Returns the soft cut duration of this [`ProjectM`].
    pub fn get_soft_cut_duration(&self) -> Duration {
        Duration::from_secs_f64(inner::get_soft_cut_duration(&self.instance))
    }

    /// Sets the soft cut duration of this [`ProjectM`].
    pub fn set_soft_cut_duration(&mut self, duration: Duration) {
        inner::set_soft_cut_duration(&mut self.instance, duration.as_secs_f64());
    }

    /// Returns the preset duration of this [`ProjectM`]. For more details, see [`ProjectM::set_preset_duration`]
    pub fn get_preset_duration(&self) -> Duration {
        Duration::from_secs_f64(inner::get_preset_duration(&self.instance))
    }

    /// Sets the preset duration of this [`ProjectM`].
    /// Once this time has elapsed, ProjectM will invoke the callback set in [`ProjectM::set_preset_switch_requested_event_callback`]
    pub fn set_preset_duration(&mut self, duration: Duration) {
        inner::set_preset_duration(&mut self.instance, duration.as_secs_f64());
    }

    /// Returns the per-pixel equation mesh size in units for this [`ProjectM`].
    /// The returned tuple is ordered as (width, height)
    pub fn get_mesh_size(&self) -> (usize, usize) {
        inner::get_mesh_size(&self.instance)
    }

    /// Sets the per-pixel equation mesh size in units.
    /// This will currently remove any active presets and reload the default \"idle\" preset.
    pub fn set_mesh_size(&mut self, mesh_x: usize, mesh_y: usize) {
        inner::set_mesh_size(&mut self.instance, mesh_x, mesh_y);
    }

    /// Gets the FPS that ProjectM expects to run at.
    /// It's your responsibility to update this with the framerate you're actually running with [`ProjectM::set_fps`]
    pub fn get_fps(&self) -> u32 {
        inner::get_fps(&self.instance)
    }

    /// Sets the FPS that ProjectM expects to run at.
    /// If you don't call this, the FPS will default to 60.
    pub fn set_fps(&mut self, fps: u32) {
        inner::set_fps(&mut self.instance, fps);
    }

    /// Returns whether or not aspect ratio correction is applied for this [`ProjectM`].
    /// For more details, see [`ProjectM::set_aspect_correction`]
    pub fn get_aspect_correction(&self) -> bool {
        inner::get_aspect_correction(&self.instance)
    }

    /// Sets if aspect ratio correction is enabled in presets that support it.
    /// This sets a flag presets can use to aspect-correct rendered shapes, which otherwise would be distorted if the viewport isn't exactly square.
    pub fn set_aspect_correction(&mut self, enabled: bool) {
        inner::set_aspect_correction(&mut self.instance, enabled);
    }

    /// Returns the current variation for preset durations (the "easter egg" value) for this [`ProjectM`].
    /// For more details, see [`ProjectM::set_preset_duration_variance`]
    pub fn get_preset_duration_variance(&self) -> f32 {
        inner::get_easter_egg(&self.instance)
    }

    /// Sets the variation for preset durations (the "easter egg" value) for this [`ProjectM`].
    ///
    /// This doesn't enable any fancy feature, it only influences the randomized display time of presets. It's
    /// passed as the "sigma" value of the gaussian random number generator used to determine the maximum display time,
    /// effectively multiplying the generated number of seconds by this amount.
    pub fn set_preset_duration_variance(&mut self, sensitivity: f32) {
        inner::set_easter_egg(&mut self.instance, sensitivity);
    }

    /// Returns whether or not the current preset is "locked" for this [`ProjectM`].
    /// For more details, see [`ProjectM::set_preset_locked`]
    pub fn get_preset_locked(&self) -> bool {
        inner::get_preset_locked(&self.instance)
    }

    /// Sets whether or not the current preset is "locked" for this [`ProjectM`].
    /// In this context, "locked" means that ProjectM won't send any requests to switch presets via
    /// [`ProjectM::set_preset_switch_requested_event_callback`]
    pub fn set_preset_locked(&mut self, lock: bool) {
        inner::set_preset_locked(&mut self.instance, lock);
    }

    /// Returns the current viewport size in pixels for this [`ProjectM`]
    /// For more details, see [`ProjectM::set_window_size`]
    pub fn get_window_size(&self) -> (usize, usize) {
        inner::get_window_size(&self.instance)
    }

    /// Sets the viewport size in pixels that this [`ProjectM`] will attempt to render at.
    /// It's the user's responsibility that this stays in sync what whatever window this is being shown in.
    pub fn set_window_size(&mut self, width: usize, height: usize) {
        inner::set_window_size(&mut self.instance, width, height);
    }

    /// Renders a frame to the OpenGL framebuffer.
    pub fn render_frame(&mut self) {
        inner::render_frame(&mut self.instance);
    }

    /// Send a touch event to the [`ProjectM`] instance. This will put a waveform of the specified [`TouchType`] on screen at the given x and y coordinates.
    pub fn touch(&mut self, x: f32, y: f32, pressure: i32, touch_type: TouchType) {
        inner::touch(&mut self.instance, x, y, pressure, touch_type.into());
    }

    /// Centers any waveforms under the coordinates to simulate dragging.
    pub fn touch_drag(&mut self, x: f32, y: f32, pressure: i32) {
        inner::touch_drag(&mut self.instance, x, y, pressure);
    }

    /// Removes any additional touch waveforms under the given coordinates.
    pub fn touch_destroy(&mut self, x: f32, y: f32) {
        inner::touch_destroy(&mut self.instance, x, y);
    }

    /// Removes all touch waveforms from the screen.
    /// Preset-defined waveforms will still be displayed.
    pub fn touch_destroy_all(&mut self) {
        inner::touch_destroy_all(&mut self.instance);
    }

    /// Returns the maximum number of audio samples that can be stored.
    /// Each PCM data UpdateMeshSize should not exceed this number of samples. If more samples are added, only this number of samples is stored and the remainder discarded.
    pub fn pcm_get_max_samples() -> u32 {
        inner::pcm_get_max_samples()
    }

    /// Adds 32-bit floating-point audio samples to projectM's internal audio buffer. It is internally converted to 2-channel float data, duplicating the channel.
    /// If stereo, the channel order in samples is LRLRLR.
    pub fn pcm_add_float(&mut self, samples: &[f32], channels: ProjectMChannels) {
        inner::pcm_add_float(&mut self.instance, samples, channels);
    }

    /// Adds 16-bit integer audio samples to projectM's internal audio buffer. It is internally converted to 2-channel float data, duplicating the channel.
    /// If stereo, the channel order in samples is LRLRLR.
    pub fn pcm_add_int16(&mut self, samples: &[i16], channels: ProjectMChannels) {
        inner::pcm_add_int16(&mut self.instance, samples, channels);
    }

    /// Adds 8-bit unsigned integer audio samples to projectM's internal audio buffer. It is internally converted to 2-channel float data, duplicating the channel.
    /// If stereo, the channel order in samples is LRLRLR.
    pub fn pcm_add_uint8(&mut self, samples: &[u8], channels: ProjectMChannels) {
        inner::pcm_add_uint8(&mut self.instance, samples, channels);
    }

    /// Writes a .bmp main texture dump after rendering the next main texture, before shaders are applied.
    /// If no file name is given, the image is written to the current working directory and will be named named "frame_texture_contents-YYYY-mm-dd-HH:MM:SS-frame.bmp".
    /// Note this is the main texture contents, not the final rendering result. If the active preset uses a composite shader, the dumped image will not have it applied. The main texture is what is passed over to the next frame, the composite shader is only applied to the display framebuffer after updating the main texture.
    /// To capture the actual output, dump the contents of the main framebuffer after calling @a projectm_render_frame() on the application side.
    pub fn write_debug_image_on_next_frame(&self, output_file: Option<&str>) {
        inner::write_debug_image_on_next_frame(&self.instance, output_file);
    }
}

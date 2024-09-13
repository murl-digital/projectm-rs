use std::{
    ffi::{CString, OsStr},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

use super::{ProjectMHandle, ProjectMTouchType};

extern crate libc;
extern crate projectm_sys as ffi;

pub use ffi::projectm;

// -----------------
// Core
// -----------------

pub(crate) fn create() -> *mut ffi::projectm {
    unsafe { ffi::projectm_create() }
}

pub(crate) unsafe fn destroy(instance: &mut ProjectMHandle) {
    ffi::projectm_destroy(instance.0)
}

pub(crate) fn load_preset_file(
    instance: &mut ProjectMHandle,
    filename: &OsStr,
    smooth_transition: bool,
) {
    unsafe {
        ffi::projectm_load_preset_file(
            instance.0,
            filename.as_bytes().as_ptr() as *const i8,
            smooth_transition,
        )
    };
}

pub(crate) fn load_preset_data(instance: &mut ProjectMHandle, data: &str, smooth_transition: bool) {
    unsafe {
        ffi::projectm_load_preset_data(instance.0, data.as_ptr() as *mut i8, smooth_transition)
    };
}

pub(crate) fn reset_textures(instance: &mut ProjectMHandle) {
    unsafe { ffi::projectm_reset_textures(instance.0) };
}

pub(crate) fn get_version_components() -> (i32, i32, i32) {
    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Version {
        major: i32,
        minor: i32,
        patch: i32,
    }

    let mut version = Version::default();

    unsafe {
        ffi::projectm_get_version_components(
            std::ptr::addr_of_mut!(version.major),
            std::ptr::addr_of_mut!(version.minor),
            std::ptr::addr_of_mut!(version.patch),
        );
    }

    (version.major, version.minor, version.patch)
}

pub(crate) fn get_version_string() -> String {
    let get_version = unsafe { ffi::projectm_get_version_string() };
    let version_str = unsafe { std::ffi::CStr::from_ptr(get_version) };
    let version_str_slice = version_str.to_str().unwrap();
    let version = version_str_slice.to_owned();

    unsafe { ffi::projectm_free_string(get_version) };

    version
}

pub(crate) fn get_vcs_version_string() -> String {
    let get_vcs_version = unsafe { ffi::projectm_get_vcs_version_string() };
    let vcs_version_str = unsafe { std::ffi::CStr::from_ptr(get_vcs_version) };
    let vcs_version_str_slice = vcs_version_str.to_str().unwrap();
    let vcs_version = vcs_version_str_slice.to_owned();

    unsafe { ffi::projectm_free_string(get_vcs_version) };

    vcs_version
}

// -----------------
// Callbacks
// -----------------

pub(crate) fn set_preset_switch_requested_event_callback<F: FnMut(bool)>(
    instance: &mut ProjectMHandle,
    callback: F,
) {
    unsafe extern "C" fn trampoline<F: FnMut(bool)>(
        is_hard_cut: bool,
        user_data: *mut std::os::raw::c_void,
    ) {
        unsafe { (*user_data.cast::<F>())(is_hard_cut) }
    }
    unsafe {
        ffi::projectm_set_preset_switch_requested_event_callback(
            instance.0,
            Some(trampoline::<F>),
            (Box::leak(Box::new(callback)) as *mut F).cast::<std::os::raw::c_void>(),
        )
    }
}

pub(crate) fn set_preset_switch_failed_event_callback<F: FnMut(String, String)>(
    instance: &mut ProjectMHandle,
    callback: F,
) {
    unsafe extern "C" fn trampoline<F: FnMut(String, String)>(
        preset_filename: *const i8,
        message: *const i8,
        user_data: *mut std::os::raw::c_void,
    ) {
        let preset_filename_str = unsafe { std::ffi::CStr::from_ptr(preset_filename) };
        let preset_filename_str_slice = preset_filename_str.to_str().unwrap();
        let preset_filename = preset_filename_str_slice.to_owned();

        let message_str = unsafe { std::ffi::CStr::from_ptr(message) };
        let message_str_slice = message_str.to_str().unwrap();
        let message = message_str_slice.to_owned();
        unsafe { (*user_data.cast::<F>())(preset_filename, message) }
    }
    unsafe {
        ffi::projectm_set_preset_switch_failed_event_callback(
            instance.0,
            Some(trampoline::<F>),
            (Box::leak(Box::new(callback)) as *mut F).cast::<std::os::raw::c_void>(),
        )
    }
}

// -----------------
// Parameters
// -----------------

pub(crate) fn set_texture_search_paths(
    instance: &mut ProjectMHandle,
    texture_search_paths: &[PathBuf],
    count: usize,
) {
    let texture_search_paths_cstr: Vec<_> = texture_search_paths
        .iter()
        .map(|arg| CString::new(arg.as_os_str().as_bytes()).unwrap())
        .collect();

    let mut texture_search_paths_pointer: Vec<_> = texture_search_paths_cstr
        .iter()
        .map(|arg| arg.as_ptr())
        .collect();

    texture_search_paths_pointer.push(std::ptr::null());

    unsafe {
        ffi::projectm_set_texture_search_paths(
            instance.0,
            texture_search_paths_pointer.as_ptr() as *mut *const ::std::os::raw::c_char,
            count,
        )
    };
}

pub(crate) fn get_beat_sensitivity(instance: &ProjectMHandle) -> f32 {
    unsafe { ffi::projectm_get_beat_sensitivity(instance.0) }
}

pub(crate) fn set_beat_sensitivity(instance: &mut ProjectMHandle, sensitivity: f32) {
    unsafe { ffi::projectm_set_beat_sensitivity(instance.0, sensitivity) };
}

pub(crate) fn get_hard_cut_duration(instance: &ProjectMHandle) -> f64 {
    unsafe { ffi::projectm_get_hard_cut_duration(instance.0) }
}

pub(crate) fn set_hard_cut_duration(instance: &mut ProjectMHandle, seconds: f64) {
    unsafe { ffi::projectm_set_hard_cut_duration(instance.0, seconds) };
}

pub(crate) fn get_hard_cut_enabled(instance: &ProjectMHandle) -> bool {
    unsafe { ffi::projectm_get_hard_cut_enabled(instance.0) }
}

pub(crate) fn set_hard_cut_enabled(instance: &mut ProjectMHandle, enabled: bool) {
    unsafe { ffi::projectm_set_hard_cut_enabled(instance.0, enabled) }
}

pub(crate) fn get_hard_cut_sensitivity(instance: &ProjectMHandle) -> f32 {
    unsafe { ffi::projectm_get_hard_cut_sensitivity(instance.0) }
}

pub(crate) fn set_hard_cut_sensitivity(instance: &mut ProjectMHandle, sensitivity: f32) {
    unsafe { ffi::projectm_set_hard_cut_sensitivity(instance.0, sensitivity) }
}

pub(crate) fn get_soft_cut_duration(instance: &ProjectMHandle) -> f64 {
    unsafe { ffi::projectm_get_soft_cut_duration(instance.0) }
}

pub(crate) fn set_soft_cut_duration(instance: &mut ProjectMHandle, seconds: f64) {
    unsafe { ffi::projectm_set_soft_cut_duration(instance.0, seconds) }
}

pub(crate) fn get_preset_duration(instance: &ProjectMHandle) -> f64 {
    unsafe { ffi::projectm_get_preset_duration(instance.0) }
}

pub(crate) fn set_preset_duration(instance: &mut ProjectMHandle, seconds: f64) {
    unsafe { ffi::projectm_set_preset_duration(instance.0, seconds) }
}

pub(crate) fn get_mesh_size(instance: &ProjectMHandle) -> (usize, usize) {
    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Mesh {
        mesh_x: usize,
        mesh_y: usize,
    }

    let mut mesh = Mesh::default();

    unsafe {
        ffi::projectm_get_mesh_size(
            instance.0,
            std::ptr::addr_of_mut!(mesh.mesh_x),
            std::ptr::addr_of_mut!(mesh.mesh_y),
        );
    }

    (mesh.mesh_x, mesh.mesh_y)
}

pub(crate) fn set_mesh_size(instance: &mut ProjectMHandle, mesh_x: usize, mesh_y: usize) {
    unsafe {
        ffi::projectm_set_mesh_size(instance.0, mesh_x, mesh_y);
    }
}

pub(crate) fn get_fps(instance: &ProjectMHandle) -> u32 {
    unsafe { ffi::projectm_get_fps(instance.0).try_into().unwrap() }
}

// FIXME: shouldn't it also be a usize?
pub(crate) fn set_fps(instance: &mut ProjectMHandle, fps: u32) {
    unsafe { ffi::projectm_set_fps(instance.0, fps as i32) };
}

pub(crate) fn get_aspect_correction(instance: &ProjectMHandle) -> bool {
    unsafe { ffi::projectm_get_aspect_correction(instance.0) }
}

pub(crate) fn set_aspect_correction(instance: &mut ProjectMHandle, enabled: bool) {
    unsafe { ffi::projectm_set_aspect_correction(instance.0, enabled) };
}

pub(crate) fn get_easter_egg(instance: &ProjectMHandle) -> f32 {
    unsafe { ffi::projectm_get_easter_egg(instance.0) }
}

pub(crate) fn set_easter_egg(instance: &mut ProjectMHandle, sensitivity: f32) {
    unsafe { ffi::projectm_set_easter_egg(instance.0, sensitivity) };
}

pub(crate) fn get_preset_locked(instance: &ProjectMHandle) -> bool {
    unsafe { ffi::projectm_get_preset_locked(instance.0) }
}

pub(crate) fn set_preset_locked(instance: &mut ProjectMHandle, lock: bool) {
    unsafe { ffi::projectm_set_preset_locked(instance.0, lock) };
}

pub(crate) fn get_window_size(instance: &ProjectMHandle) -> (usize, usize) {
    #[derive(Debug, Default, Copy, Clone)]
    #[repr(C, packed)]
    struct Mesh {
        width: usize,
        height: usize,
    }

    let mut window = Mesh::default();

    unsafe {
        ffi::projectm_get_window_size(
            instance.0,
            std::ptr::addr_of_mut!(window.width),
            std::ptr::addr_of_mut!(window.height),
        );
    }

    (window.width, window.height)
}

pub(crate) fn set_window_size(instance: &mut ProjectMHandle, width: usize, height: usize) {
    unsafe { ffi::projectm_set_window_size(instance.0, width, height) };
}

// -----------------
// Render OpenGL
// -----------------

pub(crate) fn render_frame(instance: &mut ProjectMHandle) {
    unsafe { ffi::projectm_opengl_render_frame(instance.0) };
}

// -----------------
// Touch
// -----------------

pub(crate) fn touch(
    instance: &mut ProjectMHandle,
    x: f32,
    y: f32,
    pressure: i32,
    touch_type: ProjectMTouchType,
) {
    unsafe { ffi::projectm_touch(instance.0, x, y, pressure, touch_type) };
}

pub(crate) fn touch_drag(instance: &mut ProjectMHandle, x: f32, y: f32, pressure: i32) {
    unsafe { ffi::projectm_touch_drag(instance.0, x, y, pressure) };
}

pub(crate) fn touch_destroy(instance: &mut ProjectMHandle, x: f32, y: f32) {
    unsafe { ffi::projectm_touch_destroy(instance.0, x, y) };
}

pub(crate) fn touch_destroy_all(instance: &mut ProjectMHandle) {
    unsafe { ffi::projectm_touch_destroy_all(instance.0) };
}

// -----------------
// Audio
// -----------------

pub(crate) fn pcm_get_max_samples() -> u32 {
    unsafe { ffi::projectm_pcm_get_max_samples() }
}

pub(crate) fn pcm_add_float(instance: &mut ProjectMHandle, samples: &[f32], channels: u32) {
    assert!(
        samples.len() <= pcm_get_max_samples() as usize,
        "Number of samples is greater than max samples"
    );
    let samples_per_channel = samples.len() / channels as usize;
    unsafe {
        ffi::projectm_pcm_add_float(
            instance.0,
            samples.as_ptr(),
            samples_per_channel as u32,
            channels.try_into().unwrap(),
        )
    }
}

pub(crate) fn pcm_add_int16(instance: &mut ProjectMHandle, samples: &[i16], channels: u32) {
    assert!(
        samples.len() <= pcm_get_max_samples() as usize,
        "Number of samples is greater than max samples"
    );
    let samples_per_channel = samples.len() / channels as usize;
    unsafe {
        ffi::projectm_pcm_add_int16(
            instance.0,
            samples.as_ptr(),
            samples_per_channel as u32,
            channels.try_into().unwrap(),
        )
    }
}

pub(crate) fn pcm_add_uint8(instance: &mut ProjectMHandle, samples: &[u8], channels: u32) {
    assert!(
        samples.len() <= pcm_get_max_samples() as usize,
        "Number of samples is greater than max samples"
    );
    let samples_per_channel = samples.len() / channels as usize;
    unsafe {
        ffi::projectm_pcm_add_uint8(
            instance.0,
            samples.as_ptr(),
            samples_per_channel as u32,
            channels.try_into().unwrap(),
        )
    }
}

// -----------------
// Debug
// -----------------

pub(crate) fn write_debug_image_on_next_frame(
    instance: &ProjectMHandle,
    output_file: Option<&str>,
) {
    // Transform the Rust String into a C String - this is needed due to the
    // fact that Rust Strings are not null terminated.
    let path = output_file.map(|p| {
        CString::new(p).expect("Provided output file path could not be converted to a C string")
    });

    // `path` will be alive until the end of the scope, so we can safely get
    // a pointer to it.
    let ptr = path
        .as_ref()
        .map(|s| s.as_ptr())
        .unwrap_or(std::ptr::null());

    unsafe { ffi::projectm_write_debug_image_on_next_frame(instance.0, ptr) };
}

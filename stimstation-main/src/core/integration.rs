#[cfg(not(target_arch = "wasm32"))]
use crate::audio::audio_integration::AudioIntegration;
#[cfg(not(target_arch = "wasm32"))]
use crate::text::text_processor::TextProcessor;
#[cfg(not(target_arch = "wasm32"))]
use winit::monitor::MonitorHandle;

#[cfg(not(target_arch = "wasm32"))]
static mut AUDIO_INTEGRATION: Option<AudioIntegration> = None;
#[cfg(not(target_arch = "wasm32"))]
static mut TEXT_RENDERER: Option<TextProcessor> = None;
static mut MONITOR_WIDTH: Option<u32> = None;
static mut MONITOR_HEIGHT: Option<u32> = None;

#[cfg(not(target_arch = "wasm32"))]
pub fn set_monitor_dimensions(monitor: &MonitorHandle) {
    let size = monitor.size();
    unsafe {
        MONITOR_WIDTH = Some(size.width);
        MONITOR_HEIGHT = Some(size.height);
        println!("Monitor dimensions set: {}x{}", size.width, size.height);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn set_monitor_dimensions_web(width: u32, height: u32) {
    unsafe {
        MONITOR_WIDTH = Some(width);
        MONITOR_HEIGHT = Some(height);
    }
}

pub fn get_monitor_dimensions() -> (Option<u32>, Option<u32>) {
    unsafe { (MONITOR_WIDTH, MONITOR_HEIGHT) }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn initialize_audio_integration() {
    unsafe {
        if AUDIO_INTEGRATION.is_none() {
            AUDIO_INTEGRATION = Some(AudioIntegration::new());
        }
        if let Some(audio_integration) = AUDIO_INTEGRATION.as_mut() {
            audio_integration.initialize();
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn initialize_audio_integration() {
    // Audio not supported in WASM
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_and_draw_audio(
    frame: &mut [u8],
    width: u32,
    height: u32,
    time: f32,
    x_offset: usize,
    buffer_width: u32,
) {
    unsafe {
        if let Some(audio_integration) = AUDIO_INTEGRATION.as_mut() {
            let monitor_height = MONITOR_HEIGHT;
            audio_integration.update(time, monitor_height);
            audio_integration.draw(frame, width, height, x_offset, buffer_width);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn update_and_draw_audio(
    _frame: &mut [u8],
    _width: u32,
    _height: u32,
    _time: f32,
    _x_offset: usize,
    _buffer_width: u32,
) {
    // Audio visualization not supported in WASM
}

#[cfg(not(target_arch = "wasm32"))]
pub fn initialize_text_renderer() {}

#[cfg(target_arch = "wasm32")]
pub fn initialize_text_renderer() {}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_and_draw_text(
    frame: &mut [u8],
    width: u32,
    height: u32,
    time: f32,
    x_offset: usize,
    buffer_width: u32,
) {
    unsafe {
        if let Some(text_renderer) = TEXT_RENDERER.as_mut() {
            text_renderer.update(time, width, height);
            text_renderer.draw(frame, width, height, x_offset, buffer_width);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn update_and_draw_text(
    _frame: &mut [u8],
    _width: u32,
    _height: u32,
    _time: f32,
    _x_offset: usize,
    _buffer_width: u32,
) {
    // Text rendering not supported in WASM
}

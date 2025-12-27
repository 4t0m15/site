//! Web Audio API handler for WASM builds
//! Provides audio spectrum analysis using the browser's Web Audio API

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use web_sys::{AudioContext, AnalyserNode, MediaStream};

pub const AUDIO_VIZ_BARS: usize = 64;

thread_local! {
    static AUDIO_SPECTRUM: RefCell<Vec<f32>> = RefCell::new(vec![0.0; AUDIO_VIZ_BARS]);
    static AUDIO_CONTEXT: RefCell<Option<AudioContext>> = RefCell::new(None);
    static ANALYSER: RefCell<Option<AnalyserNode>> = RefCell::new(None);
    static AUDIO_ENABLED: RefCell<bool> = RefCell::new(false);
}

/// Get the current audio spectrum data
pub fn get_audio_spectrum() -> Vec<f32> {
    AUDIO_SPECTRUM.with(|spectrum| spectrum.borrow().clone())
}

/// Check if audio is enabled
pub fn is_audio_enabled() -> bool {
    AUDIO_ENABLED.with(|enabled| *enabled.borrow())
}

/// Update the audio spectrum from the analyser node
pub fn update_audio_spectrum() {
    ANALYSER.with(|analyser_cell| {
        if let Some(analyser) = analyser_cell.borrow().as_ref() {
            let fft_size = analyser.frequency_bin_count() as usize;
            let mut frequency_data = vec![0u8; fft_size];
            analyser.get_byte_frequency_data(&mut frequency_data);
            
            AUDIO_SPECTRUM.with(|spectrum| {
                let mut spectrum = spectrum.borrow_mut();
                let bins_per_bar = fft_size / AUDIO_VIZ_BARS;
                
                for i in 0..AUDIO_VIZ_BARS {
                    let start = i * bins_per_bar;
                    let end = ((i + 1) * bins_per_bar).min(fft_size);
                    
                    let mut sum: f32 = 0.0;
                    for j in start..end {
                        sum += frequency_data[j] as f32 / 255.0;
                    }
                    
                    let avg = if end > start {
                        sum / (end - start) as f32
                    } else {
                        0.0
                    };
                    
                    // Smooth the spectrum with decay
                    spectrum[i] = spectrum[i] * 0.7 + avg * 0.3;
                    
                    // Apply bass boost for lower frequencies
                    if i < AUDIO_VIZ_BARS / 4 {
                        let boost = 1.5 * (1.0 - i as f32 / (AUDIO_VIZ_BARS / 4) as f32);
                        spectrum[i] *= 1.0 + boost;
                    }
                    
                    spectrum[i] = spectrum[i].clamp(0.0, 1.0);
                }
            });
        }
    });
}

/// Initialize audio context with microphone input
/// Note: Requires HTTPS or localhost for microphone access
#[wasm_bindgen]
pub async fn init_audio() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let navigator = window.navigator();
    
    // Check if mediaDevices is available (requires secure context)
    let media_devices = match navigator.media_devices() {
        Ok(md) => md,
        Err(_) => {
            return Err(JsValue::from_str(
                "Microphone requires HTTPS. Use 'Play Music File' instead, or access via localhost/HTTPS."
            ));
        }
    };
    
    // Create audio context
    let audio_context = AudioContext::new()?;
    
    // Create analyser node
    let analyser = audio_context.create_analyser()?;
    analyser.set_fft_size(2048);
    analyser.set_smoothing_time_constant(0.8);
    
    // Request microphone access
    let mut constraints = web_sys::MediaStreamConstraints::new();
    constraints.set_audio(&JsValue::TRUE);
    constraints.set_video(&JsValue::FALSE);
    
    let media_stream_promise = media_devices.get_user_media_with_constraints(&constraints)?;
    let media_stream: MediaStream = wasm_bindgen_futures::JsFuture::from(media_stream_promise)
        .await?
        .dyn_into()?;
    
    // Connect microphone to analyser
    let source = audio_context.create_media_stream_source(&media_stream)?;
    source.connect_with_audio_node(&analyser)?;
    
    // Store the audio context and analyser
    AUDIO_CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = Some(audio_context);
    });
    
    ANALYSER.with(|a| {
        *a.borrow_mut() = Some(analyser);
    });
    
    AUDIO_ENABLED.with(|enabled| {
        *enabled.borrow_mut() = true;
    });
    
    web_sys::console::log_1(&"Audio initialized successfully!".into());
    
    Ok(())
}

/// Initialize audio with an audio element (for playing music files)
#[wasm_bindgen]
pub fn init_audio_from_element(element_id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    
    let audio_element = document
        .get_element_by_id(element_id)
        .ok_or_else(|| JsValue::from_str("Audio element not found"))?
        .dyn_into::<web_sys::HtmlMediaElement>()?;
    
    // Create audio context
    let audio_context = AudioContext::new()?;
    
    // Create analyser node
    let analyser = audio_context.create_analyser()?;
    analyser.set_fft_size(2048);
    analyser.set_smoothing_time_constant(0.8);
    
    // Create media element source
    let source = audio_context.create_media_element_source(&audio_element)?;
    source.connect_with_audio_node(&analyser)?;
    analyser.connect_with_audio_node(&audio_context.destination())?;
    
    // Store the audio context and analyser
    AUDIO_CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = Some(audio_context);
    });
    
    ANALYSER.with(|a| {
        *a.borrow_mut() = Some(analyser);
    });
    
    AUDIO_ENABLED.with(|enabled| {
        *enabled.borrow_mut() = true;
    });
    
    web_sys::console::log_1(&"Audio from element initialized successfully!".into());
    
    Ok(())
}

/// Resume audio context (needed after user interaction)
#[wasm_bindgen]
pub fn resume_audio() -> Result<(), JsValue> {
    AUDIO_CONTEXT.with(|ctx| {
        if let Some(audio_context) = ctx.borrow().as_ref() {
            let _ = audio_context.resume();
        }
    });
    Ok(())
}


pub mod algorithms;
#[cfg(not(target_arch = "wasm32"))]
pub mod audio;
#[cfg(target_arch = "wasm32")]
pub mod web_audio;
pub mod core;
pub mod graphics;
pub mod physics;
#[cfg(not(target_arch = "wasm32"))]
pub mod text;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn run_app() -> Result<(), JsValue> {
    use web_sys::window;
    use std::cell::RefCell;
    use std::rc::Rc;
    
    web_sys::console::log_1(&"Starting StimStation...".into());
    
    // For now, we'll use a simple canvas-based approach
    // The full pixels integration requires more setup
    let window = window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id("stimstation_canvas")
        .expect("should have stimstation_canvas on the page")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("stimstation_canvas should be an HtmlCanvasElement");
    
    // Set canvas size
    canvas.set_width(types::WIDTH);
    canvas.set_height(types::HEIGHT);
    
    let context = canvas
        .get_context("2d")?
        .expect("should have 2d context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    
    // Create image data buffer
    let frame = vec![0u8; (types::WIDTH * types::HEIGHT * 4) as usize];
    
    let start_time = window.performance().expect("should have performance").now();
    
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    
    let context = Rc::new(context);
    let frame = Rc::new(RefCell::new(frame));
    
    *g.borrow_mut() = Some(Closure::new(move || {
        let window = web_sys::window().unwrap();
        let current_time = window.performance().unwrap().now();
        let elapsed = ((current_time - start_time) / 1000.0) as f32;
        
        // Update audio spectrum before drawing
        web_audio::update_audio_spectrum();
        
        // Draw frame
        {
            let mut frame_data = frame.borrow_mut();
            orchestrator::draw_frame(
                &mut frame_data,
                types::WIDTH,
                types::HEIGHT,
                elapsed,
                0,
                types::WIDTH,
            );
            
            // Convert to ImageData and draw
            let clamped = wasm_bindgen::Clamped(&frame_data[..]);
            if let Ok(image_data) = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                clamped,
                types::WIDTH,
                types::HEIGHT,
            ) {
                let _ = context.put_image_data(&image_data, 0.0, 0.0);
            }
        }
        
        // Request next frame
        let _ = window.request_animation_frame(
            f.borrow().as_ref().unwrap().as_ref().unchecked_ref()
        );
    }));
    
    // Start the animation loop
    let window = web_sys::window().unwrap();
    let _ = window.request_animation_frame(
        g.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    );
    
    Ok(())
}

// Re-export commonly used types and modules
pub use core::integration;
pub use core::orchestrator;
pub use core::types;

// App module - integrates with the orchestrator (native only)
#[cfg(not(target_arch = "wasm32"))]
pub mod app {
    use crate::integration;
    use crate::orchestrator;
    use crate::types::{HEIGHT, WIDTH};
    use std::sync::Arc;
    use std::time::Instant;
    use winit::keyboard::KeyCode;

    pub struct App {
        quit: bool,
        start_time: Instant,
    }

    impl App {
        pub fn new(window: &Arc<winit::window::Window>) -> Self {
            // Set monitor dimensions for scaling
            if let Some(monitor) = window.current_monitor() {
                integration::set_monitor_dimensions(&monitor);
            }

            Self {
                quit: false,
                start_time: Instant::now(),
            }
        }

        pub fn draw(&mut self, frame: &mut [u8]) {
            let time = self.start_time.elapsed().as_secs_f32();
            orchestrator::draw_frame(frame, WIDTH, HEIGHT, time, 0, WIDTH);
        }

        pub fn should_quit(&self) -> bool {
            self.quit
        }

        pub fn quit(&mut self) {
            self.quit = true;
        }
        pub fn handle_input(
            &mut self,
            input: &mut winit_input_helper::WinitInputHelper,
            _window: &winit::window::Window,
        ) {
            // Add input handling for physics forces, etc.
            if input.key_pressed(KeyCode::Escape) {
                self.quit();
            }

            // Toggle white noise with '9' key (native only)
            #[cfg(not(target_arch = "wasm32"))]
            if input.key_pressed(KeyCode::Digit9) {
                let enabled = !crate::audio::audio_playback::is_white_noise_enabled();
                crate::audio::audio_playback::set_white_noise_enabled(enabled);
                if enabled {
                    println!("White noise enabled");
                } else {
                    println!("White noise disabled");
                }
            }

            // Example: Add force to balls with arrow keys
            if input.key_held(KeyCode::ArrowLeft) {
                crate::physics::physics::apply_force_yellow(-0.1, 0.0);
            }
            if input.key_held(KeyCode::ArrowRight) {
                crate::physics::physics::apply_force_yellow(0.1, 0.0);
            }
            if input.key_held(KeyCode::ArrowUp) {
                crate::physics::physics::apply_force_yellow(0.0, -0.1);
            }
            if input.key_held(KeyCode::ArrowDown) {
                crate::physics::physics::apply_force_yellow(0.0, 0.1);
            }
        }
    }
}

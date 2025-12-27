use eframe::egui;
use std::time::Duration;

/// A single animation frame with its texture and display duration.
struct Frame {
    texture: egui::TextureHandle,
    duration: f32, // seconds
}

pub struct GifHandler {
    frames: Vec<Frame>,
    current_frame: usize,
    elapsed: f32,
}

impl Default for GifHandler {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            current_frame: 0,
            elapsed: 0.0,
        }
    }
}

impl GifHandler {
    pub fn load_gif_from_bytes(
        &mut self,
        ctx: &egui::Context,
        gif_bytes: &[u8],
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Clear existing frames and reset animation state
        self.frames.clear();
        self.current_frame = 0;
        self.elapsed = 0.0;

        use std::io::Cursor;
        use image::codecs::gif::GifDecoder;
        use image::AnimationDecoder;

        // Decode GIF and collect frames
        let reader = GifDecoder::new(Cursor::new(gif_bytes))?;
        let frames = reader.into_frames().collect_frames()?;

        // Upload each frame as a texture
        for (frame_index, frame) in frames.into_iter().enumerate() {
            // Extract frame delay before consuming the frame
            let delay_dur: Duration = frame.delay().into();
            let delay = delay_dur.as_secs_f32();
            // Convert frame into an ImageBuffer to get dimensions and raw data
            let buffer = frame.into_buffer(); // RGBA ImageBuffer<u8>
            let (w_u32, h_u32) = buffer.dimensions();
            let w = w_u32 as usize;
            let h = h_u32 as usize;
            let raw = buffer.into_raw(); // Vec<u8>
            let pixels = raw.chunks_exact(4)
                .map(|ch| egui::Color32::from_rgba_unmultiplied(ch[0], ch[1], ch[2], ch[3]))
                .collect::<Vec<_>>();

            let image = egui::ColorImage { size: [w, h], pixels };
            let frame_name = format!("{}_frame{}", name, frame_index);
            let texture = ctx.load_texture(frame_name, image, egui::TextureOptions::default());
            self.frames.push(Frame { texture, duration: delay });
        }

        Ok(())
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.frames.is_empty() {
            self.elapsed += delta_time;
            // advance frame while enough time has passed
            while self.elapsed >= self.frames[self.current_frame].duration && !self.frames.is_empty() {
                self.elapsed -= self.frames[self.current_frame].duration;
                self.current_frame = (self.current_frame + 1) % self.frames.len();
            }
        }
    }

    pub fn get_current_frame(&self) -> Option<&egui::TextureHandle> {
        self.frames.get(self.current_frame).map(|f| &f.texture)
    }

    pub fn render(&mut self, ui: &mut egui::Ui, size: [f32; 2]) {
        // Ensure egui repaints every frame for smooth animation
        ui.ctx().request_repaint();
        self.update(ui.input(|i| i.unstable_dt));
        
        if let Some(texture) = self.get_current_frame() {
            ui.image((texture.id(), egui::Vec2::from(size)));
        } else {
            // Show a placeholder when no GIF is loaded
            ui.vertical_centered(|ui| {
                ui.small("Duck GIF not found");
            });
        }
    }
}

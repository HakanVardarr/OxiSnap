use crate::ScreenCapturer;
use crate::error::CaptureError;
use image::{ImageBuffer, RgbaImage};
use swift_rs::{SRArray, SRObject, swift};

#[repr(C)]
pub struct CaptureResult {
    pub pixels: SRArray<u8>,
    pub width: i32,
    pub height: i32,
}

swift!(fn capture_screen_swift() -> Option<SRObject<CaptureResult>>);

pub struct MacCapturer;

#[allow(clippy::new_without_default)]
impl MacCapturer {
    pub fn new() -> Self {
        Self
    }
}

impl ScreenCapturer for MacCapturer {
    fn capture(&self) -> Result<RgbaImage, CaptureError> {
        let result = unsafe { capture_screen_swift() };

        match result {
            Some(capture_data) => {
                let width = capture_data.width as u32;
                let height = capture_data.height as u32;

                let mut pixels_vec = capture_data.pixels.to_vec();

                for chunk in pixels_vec.chunks_exact_mut(4) {
                    chunk.swap(0, 2);
                }

                ImageBuffer::from_raw(width, height, pixels_vec)
                    .ok_or(CaptureError::BufferCreationError)
            }
            None => Err(CaptureError::CaptureFailed),
        }
    }
}

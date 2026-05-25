mod ffi;

use image::RgbaImage;

use crate::capturer::ScreenCapturer;
use crate::error::CaptureError;
use crate::image::{bgra_to_rgba, into_rgba_image};

pub struct MacCapturer;

impl MacCapturer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MacCapturer {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenCapturer for MacCapturer {
    fn capture(&self) -> Result<RgbaImage, CaptureError> {
        let capture_data = ffi::call_capture_screen()?;
        let width = capture_data.width as u32;
        let height = capture_data.height as u32;
        let mut pixels = capture_data.pixels.to_vec();
        bgra_to_rgba(&mut pixels);
        into_rgba_image(width, height, pixels)
    }
}

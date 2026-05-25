pub mod error;

pub mod platforms;

use image::RgbaImage;

pub trait ScreenCapturer {
    fn capture(&self) -> Result<RgbaImage, error::CaptureError>;
}

#[cfg(target_os = "macos")]
pub type NativeCapturer = platforms::macos::MacCapturer;

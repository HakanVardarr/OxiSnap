use image::RgbaImage;

use crate::error::CaptureError;

pub trait ScreenCapturer {
    fn capture(&self) -> Result<RgbaImage, CaptureError>;
}

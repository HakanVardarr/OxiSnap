use swift_rs::{SRArray, SRObject, swift};

use crate::error::FfiError;

#[repr(C)]
pub struct RawCaptureResult {
    pub pixels: SRArray<u8>,
    pub width: i32,
    pub height: i32,
}

swift!(fn capture_screen_swift() -> Option<SRObject<RawCaptureResult>>);

pub fn call_capture_screen() -> Result<SRObject<RawCaptureResult>, FfiError> {
    // SAFETY: `capture_screen_swift` is a C-ABI symbol exported by the Swift
    // `ScreenCaptureBridge` library via `@_cdecl`. It is safe to call from a single
    // thread and signals failure by returning `None` rather than panicking.
    unsafe { capture_screen_swift() }.ok_or(FfiError::NullPointer)
}

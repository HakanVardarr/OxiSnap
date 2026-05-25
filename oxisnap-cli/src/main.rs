use std::io::Cursor;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use image::ImageFormat;
use oxisnap_core::{NativeCapturer, ScreenCapturer};
use rdev::{Event, EventType, Key, listen};

struct Modifiers {
    ctrl: bool,
    shift: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend_url = std::env::var("OXISNAP_BACKEND")
        .unwrap_or_else(|_| "http://127.0.0.1:3000/upload".to_string());

    let (tx, rx) = mpsc::channel::<()>();
    let mods = Arc::new(Mutex::new(Modifiers {
        ctrl: false,
        shift: false,
    }));
    let mods_cb = Arc::clone(&mods);

    thread::spawn(move || {
        listen(move |event: Event| {
            let mut m = mods_cb.lock().expect("modifier state poisoned");
            match event.event_type {
                EventType::KeyPress(Key::ControlLeft)
                | EventType::KeyPress(Key::ControlRight) => {
                    m.ctrl = true;
                }
                EventType::KeyRelease(Key::ControlLeft)
                | EventType::KeyRelease(Key::ControlRight) => {
                    m.ctrl = false;
                }
                EventType::KeyPress(Key::ShiftLeft)
                | EventType::KeyPress(Key::ShiftRight) => {
                    m.shift = true;
                }
                EventType::KeyRelease(Key::ShiftLeft)
                | EventType::KeyRelease(Key::ShiftRight) => {
                    m.shift = false;
                }
                EventType::KeyPress(Key::KeyS) if m.ctrl && m.shift => {
                    tx.send(()).ok();
                }
                _ => {}
            }
        })
        .expect("key listener failed — grant Accessibility access in System Settings > Privacy & Security");
    });

    let client = reqwest::blocking::Client::new();
    let capturer = NativeCapturer::new();

    for _ in rx {
        eprintln!("Capturing...");
        match capture_and_upload(&capturer, &client, &backend_url) {
            Ok(link) => {
                copy_to_clipboard(&link);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

fn capture_and_upload(
    capturer: &NativeCapturer,
    client: &reqwest::blocking::Client,
    backend_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let img = capturer.capture()?;

    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)?;

    let link = client
        .post(backend_url)
        .header("Content-Type", "image/png")
        .body(png_bytes)
        .send()?
        .error_for_status()?
        .text()?;

    Ok(link)
}

fn copy_to_clipboard(text: &str) {
    match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(text)) {
        Ok(()) => {}
        Err(e) => eprintln!("Warning: clipboard copy failed: {}", e),
    }
}

use oxisnap_core::{NativeCapturer, ScreenCapturer};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CLI: Ekran yakalama aracı başlatılıyor...");
    let start_time = Instant::now();

    let capturer = NativeCapturer::new();
    let img = capturer.capture()?;

    let filename = "screenshot.png";
    img.save(filename)?;

    let duration = start_time.elapsed();
    println!(
        "Başarılı! Görüntü '{}' olarak kaydedildi. (Süre: {:.2?})",
        filename, duration
    );

    Ok(())
}

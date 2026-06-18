//! Simple waveform renderer that produces a monochrome PPM (P6) image buffer.
//! Returns a Vec<u8> with PPM bytes.

pub fn render_waveform_ppm(waveform: &[f32], width: usize, height: usize) -> Vec<u8> {
    let mut buf = Vec::new();
    // P6 header
    buf.extend_from_slice(format!("P6 {} {} 255\n", width, height).as_bytes());
    if width == 0 || height == 0 || waveform.is_empty() {
        // empty image data
        return buf;
    }
    // For each column, map waveform index and draw vertical line of intensity
    for x in 0..width {
        let idx = x * waveform.len() / width;
        let v = waveform[idx].clamp(0.0, 1.0);
        let line_h = (v * (height as f32 - 1.0)).round() as usize;
        for y in 0..height {
            let intensity = if y == (height - 1 - line_h) {
                255u8
            } else {
                0u8
            };
            // RGB
            buf.push(intensity);
            buf.push(intensity);
            buf.push(intensity);
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_small_waveform() {
        let wf = vec![0.0f32, 1.0, 0.5, 0.2];
        let width = 4;
        let height = 8;
        let img = render_waveform_ppm(&wf, width, height);
        // Basic sanity checks: starts with P6 and contains expected pixel data length
        assert!(img.starts_with(b"P6") || img.starts_with(b"P6 ") || img.starts_with(b"P6\n"));
        assert!(img.len() >= width * height * 3);
        let pixel_bytes = &img[img.len() - (width * height * 3)..];
        assert_eq!(pixel_bytes.len(), width * height * 3);
    }
}

use std::path::Path;
use std::{fs, io};

/// Initialize cache directories under `base` for bpm, waveforms, stems, and thumbnails.
pub fn init_cache_dirs(base: &Path) -> io::Result<()> {
    let dirs = ["bpm", "waveforms", "stems", "thumbnails"];
    for d in dirs {
        let p = base.join(d);
        fs::create_dir_all(&p)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn create_cache_dirs() -> Result<(), Box<dyn std::error::Error>> {
        let base = temp_dir().join(format!("lazydj-cache-test-{}", uuid::Uuid::new_v4()));
        init_cache_dirs(&base)?;
        assert!(base.join("bpm").exists());
        assert!(base.join("waveforms").exists());
        assert!(base.join("stems").exists());
        assert!(base.join("thumbnails").exists());
        // Clean up
        std::fs::remove_dir_all(&base)?;
        Ok(())
    }
}

use lazydj_decoder::DecodedTrack;
use lazydj_output::{AudioOutput, MockOutput};
use std::sync::Arc;

pub struct Player {
    output: Arc<dyn AudioOutput>,
    decoded: Option<DecodedTrack>,
    pub volume: f32,
}

impl Player {
    pub fn new(output: Arc<dyn AudioOutput>) -> Self {
        Self {
            output,
            decoded: None,
            volume: 1.0,
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let decoded = lazydj_decoder::decode_wav(path)?;
        self.decoded = Some(decoded);
        Ok(())
    }

    pub fn play(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(decoded) = &self.decoded {
            self.output.start(decoded.sample_rate, decoded.channels)?;
            let mut samples = decoded.samples.clone();
            if (self.volume - 1.0).abs() > f32::EPSILON {
                for s in &mut samples {
                    *s *= self.volume;
                }
            }
            self.output.write_samples(&samples);
            self.output.stop();
            Ok(())
        } else {
            Err("no track loaded".into())
        }
    }

    pub fn set_volume(&mut self, v: f32) {
        self.volume = v;
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Arc::new(MockOutput::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Arc;

    #[test]
    fn player_load_play_flow() {
        let pth = std::env::temp_dir().join("lazydj_player_test.wav");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        {
            let mut writer = hound::WavWriter::create(&pth, spec).unwrap();
            for s in [1000i16, -1000i16, 3000i16, -3000i16].iter() {
                writer.write_sample(*s).unwrap();
            }
            writer.finalize().unwrap();
        }

        let out = Arc::new(lazydj_output::MockOutput::new());
        let mut player = Player::new(out.clone());
        player.load(&pth).unwrap();
        player.set_volume(0.5);
        player.play().unwrap();

        let buf = out.buffer();
        assert_eq!(buf.len(), 4);
        // scaled values exist
        assert!(buf[0].abs() > 0.0);

        let _ = fs::remove_file(pth);
    }
}

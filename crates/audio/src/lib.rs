use lazydj_decoder::DecodedTrack;
use lazydj_mixer::Mixer;
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

/// MixerPlayer mixes two decoded tracks using the Mixer type and writes mixed output.
pub struct MixerPlayer {
    output: Arc<dyn AudioOutput>,
    decoded_a: Option<DecodedTrack>,
    decoded_b: Option<DecodedTrack>,
    pub mixer: Mixer,
    pub volume_a: f32,
    pub volume_b: f32,
}

impl MixerPlayer {
    pub fn new(output: Arc<dyn AudioOutput>) -> Self {
        Self {
            output,
            decoded_a: None,
            decoded_b: None,
            mixer: Mixer::new(),
            volume_a: 1.0,
            volume_b: 1.0,
        }
    }

    pub fn load_a<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let decoded = lazydj_decoder::decode_wav(path)?;
        self.decoded_a = Some(decoded);
        Ok(())
    }

    pub fn load_b<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let decoded = lazydj_decoder::decode_wav(path)?;
        self.decoded_b = Some(decoded);
        Ok(())
    }

    pub fn set_crossfade(&mut self, v: f32) {
        self.mixer.set_crossfade(v);
    }

    pub fn set_volumes(&mut self, a: f32, b: f32) {
        self.volume_a = a;
        self.volume_b = b;
    }

    pub fn play(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let (Some(a), Some(b)) = (&self.decoded_a, &self.decoded_b) {
            if a.sample_rate != b.sample_rate || a.channels != b.channels {
                return Err("sample rate or channel mismatch".into());
            }
            self.output.start(a.sample_rate, a.channels)?;

            let mut sa = a.samples.clone();
            let mut sb = b.samples.clone();

            if (self.volume_a - 1.0).abs() > f32::EPSILON {
                for s in &mut sa {
                    *s *= self.volume_a;
                }
            }
            if (self.volume_b - 1.0).abs() > f32::EPSILON {
                for s in &mut sb {
                    *s *= self.volume_b;
                }
            }

            let mixed = self.mixer.mix(&sa, &sb);
            self.output.write_samples(&mixed);
            self.output.stop();
            Ok(())
        } else {
            Err("both tracks not loaded".into())
        }
    }
}

impl Default for MixerPlayer {
    fn default() -> Self {
        MixerPlayer::new(Arc::new(MockOutput::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Arc;

    #[test]
    fn mixer_player_load_play_flow() {
        let pth_a = std::env::temp_dir().join("lazydj_mixer_player_a.wav");
        let pth_b = std::env::temp_dir().join("lazydj_mixer_player_b.wav");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        {
            let mut writer = hound::WavWriter::create(&pth_a, spec).unwrap();
            for s in [1000i16, -1000i16, 3000i16, -3000i16].iter() {
                writer.write_sample(*s).unwrap();
            }
            writer.finalize().unwrap();
        }
        {
            let mut writer = hound::WavWriter::create(&pth_b, spec).unwrap();
            for s in [0i16, 2000i16, -2000i16, 0i16].iter() {
                writer.write_sample(*s).unwrap();
            }
            writer.finalize().unwrap();
        }

        let out = Arc::new(lazydj_output::MockOutput::new());
        let mut player = MixerPlayer::new(out.clone());
        player.load_a(&pth_a).unwrap();
        player.load_b(&pth_b).unwrap();
        player.set_crossfade(0.5);
        player.set_volumes(1.0, 1.0);
        player.play().unwrap();

        let buf = out.buffer();
        // mixed buffer length matches min length
        assert_eq!(buf.len(), 4);
        assert!(buf[0].abs() > 0.0);

        let _ = fs::remove_file(pth_a);
        let _ = fs::remove_file(pth_b);
    }
}

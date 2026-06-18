use std::path::Path;

#[derive(Debug, Clone)]
pub struct DecodedTrack {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

pub fn decode_wav<P: AsRef<Path>>(
    path: P,
) -> Result<DecodedTrack, Box<dyn std::error::Error + Send + Sync>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let channels = spec.channels;
    let bits_per_sample = spec.bits_per_sample;

    let mut samples = Vec::new();

    match spec.sample_format {
        hound::SampleFormat::Int => {
            if bits_per_sample == 16 {
                for s in reader.samples::<i16>() {
                    let s = s?;
                    samples.push(s as f32 / i16::MAX as f32);
                }
            } else {
                for s in reader.samples::<i32>() {
                    let s = s?;
                    samples.push(s as f32 / i32::MAX as f32);
                }
            }
        }
        hound::SampleFormat::Float => {
            for s in reader.samples::<f32>() {
                let s = s?;
                samples.push(s);
            }
        }
    }

    Ok(DecodedTrack {
        samples,
        sample_rate,
        channels,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn decode_wav_roundtrip() {
        let p = std::env::temp_dir().join("lazydj_test_decode.wav");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        {
            let mut writer = hound::WavWriter::create(&p, spec).unwrap();
            for s in [0i16, 16384, -16384, 32767].iter() {
                writer.write_sample(*s).unwrap();
            }
            writer.finalize().unwrap();
        }

        let decoded = decode_wav(&p).unwrap();
        assert_eq!(decoded.sample_rate, 44100);
        assert_eq!(decoded.channels, 1);
        assert_eq!(decoded.samples.len(), 4);
        // check normalized ranges
        assert!(decoded.samples[0].abs() < 1e-6);

        let _ = fs::remove_file(p);
    }
}

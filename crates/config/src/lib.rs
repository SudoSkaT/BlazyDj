use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub audio_device: Option<String>,
    pub sample_rate: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            audio_device: None,
            sample_rate: Some(48000),
        }
    }
}

pub fn from_str(s: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config() {
        let s = r#"audio_device = "hw:0"
sample_rate = 96000
"#;
        let cfg = from_str(s).unwrap();
        assert_eq!(cfg.audio_device, Some("hw:0".into()));
        assert_eq!(cfg.sample_rate, Some(96000));
    }
}

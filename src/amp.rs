use std::sync::Arc;

use std::sync::RwLock;

use serde::Deserialize;
use serde::Serialize;

pub struct Amp {
    pub config: Arc<RwLock<AmpConfig>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AmpConfig {
    pub gain: u8,
}

impl Amp {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(AmpConfig { gain: 127 })),
        }
    }

    pub fn amplify(&self, _audio_in: &mut [u32]) -> anyhow::Result<u8> {
        let max = std::u32::MAX as f32;
        let c = self.config.read().unwrap();
        let v = c.gain;
        let f = (v as f32) / 128.0;
        for x in _audio_in {
            *x = ((*x as f32) * f).min(max) as u32;
        }
        Ok(v)
    }
}

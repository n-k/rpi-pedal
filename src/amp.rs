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
        let c = self.config.read().unwrap();
        let v = c.gain;
        Ok(v)
    }
}

use std::sync::Arc;

use std::sync::RwLock;

pub struct  Amp {
    pub config: Arc<RwLock<Vec<u8>>>,
}

impl Amp {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(vec![1, 2, 3, 4])),
        }
    }

    pub fn amplify(&self, _audio_in: &mut [u32]) -> anyhow::Result<usize> {
        let c = self.config.read().unwrap();
        let v = c.len();
        Ok(v)
    }
}

use std::fs::File;
use std::path::Path;

use anyhow::{Context as ErrorContext, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {}

impl Config {
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config_file = File::open(config_path).context("could not open config file")?;
        Ok(serde_json::from_reader(config_file)?)
    }

    pub fn load_or_init<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        match Self::load(&config_path) {
            Ok(state) => Ok(state),
            Err(e) => {
                if e.downcast_ref::<std::io::Error>().map(|e| e.kind())
                    == Some(std::io::ErrorKind::NotFound)
                {
                    let config = Self::default();
                    config.save(&config_path)?;
                    Ok(config)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn save<P: AsRef<Path>>(&self, config_path: P) -> Result<()> {
        let config_file = File::create(&config_path).context("could not create config file")?;
        Ok(serde_json::to_writer_pretty(config_file, self)?)
    }
}

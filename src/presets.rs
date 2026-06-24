use crate::props::APP_NAME;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;

pub trait PresetsStore {
    fn load(&mut self) -> Result<Vec<Preset>, Error>;
    fn save(&mut self, values: &Vec<Preset>) -> Result<(), Error>;
}

#[derive(Default)]
pub struct FilePresetsStore {}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Preset {
    pub command: String,
    pub shortcut: Option<char>,
}

impl PresetsStore for FilePresetsStore {
    fn load(&mut self) -> Result<Vec<Preset>, Error> {
        let mut presets = Vec::new();
        if let Some(path) = presets_path() {
            debug!("reading presets from file: {:?}", path);

            if let Ok(content) = std::fs::read_to_string(path) {
                match toml::from_str::<PresetsTomlArray>(&content) {
                    Ok(preset_wrapper) => presets = preset_wrapper.preset,
                    Err(e) => {
                        error!("failed to parse presets: {:?}", e);
                    }
                }
            }
        }

        Ok(presets)
    }

    fn save(&mut self, values: &Vec<Preset>) -> Result<(), Error> {
        if let Some(path) = presets_path() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            match std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
            {
                Ok(mut file) => {
                    let arr = PresetsTomlArray {
                        preset: (*values.clone()).to_vec(),
                    };
                    match toml::to_string(&arr) {
                        Ok(string) => {
                            debug!("saving presets to file {:?}", string);
                            let _ = writeln!(file, "{}", string);
                            Ok(())
                        }
                        Err(e) => {
                            error!("Failed to serialize presets: {}", e);
                            Err(Error::new(std::io::ErrorKind::Other, e))
                        }
                    }
                }
                Err(e) => {
                    debug!("Failed to open presets file for writing: {}", e);
                    Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to open presets file for writing",
                    ))
                }
            }
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Presets path not found",
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PresetsTomlArray {
    preset: Vec<Preset>,
}

fn presets_path() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(APP_NAME).join("presets.toml"))
}

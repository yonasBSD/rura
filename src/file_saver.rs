use anyhow::Result;
use itertools::Itertools;
use log::debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub trait FileSaver {
    fn save(&self, path: PathBuf, content: Vec<u8>) -> Result<()>;
    fn save_script(&self, path: PathBuf, content: Vec<u8>) -> Result<()>;
}

pub struct FileSavers;

impl FileSavers {
    pub fn new(shell: &str) -> Box<dyn FileSaver> {
        #[cfg(unix)]
        return Box::new(UnixFileSaver {
            shell: shell.into(),
        });
        #[cfg(windows)]
        return Box::new(WindowsFileSaver {});
    }
}

#[cfg(unix)]
struct UnixFileSaver {
    shell: String,
}

#[cfg(unix)]
impl FileSaver for UnixFileSaver {
    fn save(&self, path: PathBuf, content: Vec<u8>) -> Result<()> {
        UnixFileSaver::save(path, content, 0o644)
    }

    fn save_script(&self, path: PathBuf, content: Vec<u8>) -> Result<()> {
        let shebang = format!("#!/usr/bin/env {}\n\n", self.shell);
        let full_content = shebang
            .bytes()
            .into_iter()
            .chain(content.into_iter())
            .collect_vec();
        UnixFileSaver::save(path, full_content, 0o755)
    }
}

#[cfg(unix)]
impl UnixFileSaver {
    fn save(path: PathBuf, content: Vec<u8>, mode: u32) -> Result<()> {
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(mode)
            .open(&path)?;

        debug!("Saving with len: {:?}", content.len());

        file.write_all(&content)?;

        debug!("Successfully saved file: {:?}", path);

        Ok(())
    }
}

#[cfg(windows)]
struct WindowsFileSaver {}

#[cfg(windows)]
impl FileSaver for WindowsFileSaver {
    fn save(&self, path: PathBuf, content: Vec<u8>) -> Result<()> {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)?;

        debug!("Saving with len: {:?}", content.len());

        file.write_all(&content)?;

        debug!("Successfully saved file: {:?}", path);

        Ok(())
    }

    fn save_script(&self, path: PathBuf, content: Vec<u8>) -> Result<()> {
        self.save(path, content)
    }
}

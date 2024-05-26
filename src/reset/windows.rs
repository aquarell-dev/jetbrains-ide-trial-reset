use std::io;
use std::path::PathBuf;

use dirs::data_dir;

use crate::reset::Resetter;

pub struct WindowsResetter;

impl Resetter for WindowsResetter {
    fn get_jetbrains_directory(&self) -> io::Result<PathBuf> {
        if let Some(app_dir) = data_dir() {
            Ok(app_dir.join("JetBrains"))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "App Data directory not found!"))
        }
    }
}
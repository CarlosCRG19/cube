use crate::session::Session;
use crate::Result;

use std::{fs, io, path::PathBuf};

pub trait Storage {
    fn save_session(&self, session: &Session) -> Result<()>;
    fn load_session(&self) -> Result<Session>;
}

const DIR: &str = ".cube";
const FILE: &str = "sessions.json";

pub struct FileSystemStorage {
    file_path: PathBuf
}

impl FileSystemStorage {
    pub fn build() -> Result<Self> {
        let file_path = Self::setup_file_path()?;
        Ok(Self { file_path })
    }

    fn setup_file_path() -> Result<PathBuf> {
        let mut path = dirs::data_local_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find local data directory"))?;
        path.push(DIR);
        path.push(FILE);
        fs::create_dir_all(&path.parent().unwrap())?;
        Ok(path)
    }
}

impl Storage for FileSystemStorage {
    fn save_session(&self, session: &Session) -> Result<()> {
        let json = serde_json::to_string(session)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn load_session(&self) -> Result<Session> {
        if self.file_path.exists() {
            let json = fs::read_to_string(&self.file_path)?;
            Ok(serde_json::from_str(&json)?)
        } else {
            Ok(Session::new())
        }
    }
}

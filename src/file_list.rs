use crate::{path::WrmPath, Error::WrmError, Result};
use filey::Filey;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileInfo {
    path: String,
    path_trash: String,
}

impl FileInfo {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().display().to_string();
        let path_trash = format!(
            "{}/{}",
            WrmPath::default().expanded()?.trash(),
            Filey::new(&path)
                .file_name()
                .unwrap_or_else(|| path.to_string())
        );
        let fileinfo = FileInfo { path, path_trash };
        Ok(fileinfo)
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }

    pub fn path_trash(&self) -> String {
        self.path_trash.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileList {
    files: Vec<FileInfo>,
}

impl FileList {
    pub fn new() -> Self {
        FileList { files: vec![] }
    }

    pub fn files(&self) -> &Vec<FileInfo> {
        &self.files
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = File::open(path).map_err(|e| e.into()).map_err(WrmError)?;
        let files = serde_json::from_reader(f)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        Ok(files)
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        serde_json::to_writer_pretty(f, &self)
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        Ok(())
    }

    pub fn add(&mut self, fileinfo: &FileInfo) -> &Self {
        self.files.push(fileinfo.clone());
        self
    }

    pub fn remove(&mut self, fileinfo: &FileInfo) -> &Self {
        self.files.retain(|x| x != fileinfo);
        self
    }
}

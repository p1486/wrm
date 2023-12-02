use crate::{Error::WrmError, Result};
use filey::Filey;

pub struct WrmPath {
    dir: String,
    trash: String,
    list: String,
}

impl Default for WrmPath {
    fn default() -> Self {
        WrmPath {
            dir: "~/.config/wrm".to_string(),
            trash: "~/.config/wrm/trash".to_string(),
            list: "~/.config/wrm/list.json".to_string(),
        }
    }
}

impl WrmPath {
    pub fn dir(&self) -> &String {
        &self.dir
    }

    pub fn trash(&self) -> &String {
        &self.trash
    }

    pub fn list(&self) -> &String {
        &self.list
    }

    pub fn expanded(&self) -> Result<Self> {
        let expand = |s| -> Result<String> {
            let result = Filey::new(s)
                .expand_user()
                .map_err(|e| e.into())
                .map_err(WrmError)?
                .to_string();
            Ok(result)
        };
        let dir = expand(&self.dir)?;
        let trash = expand(&self.trash)?;
        let list = expand(&self.list)?;
        let wrm = WrmPath { dir, trash, list };
        Ok(wrm)
    }
}

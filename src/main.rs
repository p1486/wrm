// wrm
//
// wrm is a file deletion utility.
mod actions;
mod path;
mod argparse;
mod file_list;
mod test;

use crate::{path::WrmPath, argparse::argparse, file_list::FileList};
use filey::{self, create, Error::FileyError, FileTypes, Filey};
use std::process::exit;

fn main() {
    match WrmPath::default().expanded() {
        Ok(wrm_path) => {
            if let Err(e) = prepare(&wrm_path) {
                eprintln!("wrm: {}", e);
                exit(1)
            }
            if let Err(e) = argparse(&wrm_path) {
                eprintln!("wrm: {}", e);
                exit(1)
            }
        }
        Err(e) => {
            eprintln!("wrm: {}", e);
            exit(1)
        }
    }
}

// Create $HOME/.config/wrm, $HOME/.config/wrm/trash and $HOME/.config/wrm/list.json.
fn prepare(wrm_path: &WrmPath) -> filey::Result<()> {
    create!(FileTypes::Directory, wrm_path.dir(), wrm_path.trash());
    let filelist = Filey::new(wrm_path.list());
    if !filelist.exists() {
        filelist.create(FileTypes::File)?;
        FileList::new()
            .write(wrm_path.list())
            .map_err(|e| e.into())
            .map_err(FileyError)?;
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    WrmError(anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

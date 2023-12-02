use crate::{
    path::WrmPath,
    file_list::{FileInfo, FileList},
    Error::WrmError,
    Result,
};
use colored::Colorize;
use filey::{remove, FileTypes, Filey};
use std::io::{stdin, stdout, Write};

// Prompt before every actions.
fn confirm<S: AsRef<str>>(message: S) -> Result<bool> {
    let mut s = String::new();
    print!("{}", message.as_ref());
    stdout().flush().map_err(|e| e.into()).map_err(WrmError)?;
    stdin()
        .read_line(&mut s)
        .map_err(|e| e.into())
        .map_err(WrmError)?;
    let result = s.trim().to_lowercase();
    let result = result.as_str();
    if result == "y" || result == "yes " {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn call<F, S: AsRef<str>>(f: F, message: S, noninteractive: bool) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    if !noninteractive {
        if confirm(message)? {
            f()?;
        } else {
            eprintln!("Canceled");
        }
    } else {
        f()?;
    }
    Ok(())
}

// Move files or directories to trash(~/.config/wrm/trash)
pub fn remove(
    path: Vec<String>,
    wrm_path: &WrmPath,
    noninteractive: bool,
    concise: bool,
) -> Result<()> {
    for i in path {
        let p = Filey::new(i)
            .absolutized()
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        let file_type = p.file_type().map_err(|e| e.into()).map_err(WrmError)?;
        let fileinfo = FileInfo::new(p.path())?;
        let remove_closure = || -> Result<()> {
            if p.file_type().map_err(|e| e.into()).map_err(WrmError)? == FileTypes::Symlink {
                p.remove().map_err(|e| e.into()).map_err(WrmError)?;
            } else {
                p.move_to(wrm_path.trash())
                    .map_err(|e| e.into())
                    .map_err(WrmError)?;
                let mut list = FileList::read(wrm_path.list())?;
                list.add(&fileinfo).write(wrm_path.list())?;
            }
            if !concise {
                eprintln!("{} {} '{}'", "Removed".green().bold(), file_type, &p);
            }
            Ok(())
        };
        let message = format!("{} {} '{}'? [y/N] ", "Remove".red().bold(), file_type, &p);
        call(remove_closure, message, noninteractive)?;
    }
    Ok(())
}

// Delete all files and directories in trash permanently
pub fn clean(wrm_path: &WrmPath, noninteractive: bool, concise: bool) -> Result<()> {
    let filelist = FileList::read(wrm_path.list())?;
    if !filelist.files().is_empty() {
        let clean_closure = || {
            remove!(wrm_path.trash(), wrm_path.list());
            if !concise {
                eprintln!("{} trash", "Cleaned".green().bold());
            };
        };
        if !noninteractive {
            let message = format!(
                "{} these files and directories? [y/N] ",
                "Delete".red().bold()
            );
            list(wrm_path)?;
            if confirm(message)? {
                clean_closure();
            } else {
                eprintln!("Canceled")
            }
        } else {
            clean_closure();
        }
    } else {
        eprintln!("There are no files or directories in trash");
    }
    Ok(())
}

// Delete files or directories
pub fn delete(
    path: Vec<String>,
    wrm_path: &WrmPath,
    noninteractive: bool,
    concise: bool,
) -> Result<()> {
    for i in path {
        let p = Filey::new(i)
            .absolutized()
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        let file_type = p.file_type().map_err(|e| e.into()).map_err(WrmError)?;
        let delete_closure = || -> Result<()> {
            p.remove().map_err(|e| e.into()).map_err(WrmError)?;
            if !concise {
                eprintln!("{} {} '{}'", "Deleted".green().bold(), file_type, &p);
            }
            Ok(())
        };
        let message = format!("{} {} '{}'? [y/N] ", "Delete".red().bold(), file_type, &p);
        call(delete_closure, message, noninteractive)?;
        check(wrm_path)?;
    }
    Ok(())
}

// Restore files or directories in trash to where they came from
pub fn restore(
    path: Vec<String>,
    wrm_path: &WrmPath,
    noninteractive: bool,
    concise: bool,
) -> Result<()> {
    for i in path {
        let p = Filey::new(i)
            .absolutized()
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        let restore_closure = || -> Result<()> {
            let filelist = FileList::read(wrm_path.list())?;
            for i in filelist.files() {
                let q = Filey::new(i.path())
                    .absolutized()
                    .map_err(|e| e.into())
                    .map_err(WrmError)?;
                let o = Filey::new(i.path_trash())
                    .absolutized()
                    .map_err(|e| e.into())
                    .map_err(WrmError)?;
                let file_type = o.file_type().map_err(|e| e.into()).map_err(WrmError)?;
                if o.path() == p.path() {
                    o.move_to(q.path())
                        .map_err(|e| e.into())
                        .map_err(WrmError)?;
                    if !concise {
                        eprintln!(
                            "{} {} '{}' to '{}'",
                            "Restored".green().bold(),
                            file_type,
                            o,
                            q
                        );
                    }
                    break;
                }
            }
            Ok(())
        };
        let message = format!(
            "{} {} '{}'? [y/N] ",
            "Restore".red().bold(),
            &p.file_type().map_err(|e| e.into()).map_err(WrmError)?,
            &p
        );
        call(restore_closure, message, noninteractive)?;
        check(wrm_path)?;
    }
    Ok(())
}

// List all files and directories in trash
pub fn list(wrm_path: &WrmPath) -> Result<()> {
    let filelist = FileList::read(wrm_path.list())?;
    if filelist.files().is_empty() {
        eprintln!("There are no files or directories in trash");
    } else {
        for i in filelist.files() {
            let p = Filey::new(&i.path_trash());
            let file_name = p.file_name().unwrap_or_else(|| p.to_string());
            match p.file_type().map_err(|e| e.into()).map_err(WrmError)? {
                FileTypes::File => {
                    println!("{} ({}) {}", file_name, i.path(), FileTypes::File);
                }
                FileTypes::Directory => {
                    println!(
                        "{} ({}) {}",
                        file_name.blue(),
                        i.path(),
                        FileTypes::Directory
                    );
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn check(wrm_path: &WrmPath) -> Result<()> {
    let filelist = FileList::read(wrm_path.list())?;
    for i in filelist.files() {
        let p = Filey::new(&i.path_trash())
            .absolutized()
            .map_err(|e| e.into())
            .map_err(WrmError)?;
        if !p.exists() {
            filelist.clone().remove(&i.clone()).write(wrm_path.list())?;
        }
    }
    Ok(())
}

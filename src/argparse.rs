use crate::{
    actions::{clean, delete, list, remove, restore},
    Error::WrmError,
    Result, WrmPath,
};
use clap::{error::ErrorKind, Command, Parser};

/// wrm
///
/// A file deletion utility
/// If you do not use the option --delete,
/// file will be moved to trash($HOME/.config/wrm/trash) by default.
#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"), arg_required_else_help = true, verbatim_doc_comment)]
struct Args {
    file: Option<Vec<String>>,
    /// Delete all files and directories in trash permanently.
    #[clap(short, long)]
    clean: bool,
    /// Delete files or directories.
    #[clap(short, long)]
    delete: bool,
    /// List all files and directories in trash.
    #[clap(short, long)]
    list: bool,
    /// Restore files or directories in trash to where they came from.
    #[clap(short, long)]
    restore: bool,
    /// Do not prompt before every actions.
    #[clap(short, long)]
    noninteractive: bool,
    /// Do not explain what is being done.
    #[clap(short, long)]
    quiet: bool,
}

pub fn argparse(wrm_path: &WrmPath) -> Result<()> {
    let args = Args::parse();
    if args.clean {
        clean(wrm_path, args.noninteractive, args.quiet)?;
    } else if args.list {
        list(wrm_path)?;
    } else if let Some(path) = args.file {
        if args.delete {
            delete(path, wrm_path, args.noninteractive, args.quiet)?
        } else if args.restore {
            restore(path, wrm_path, args.noninteractive, args.quiet)?
        } else {
            remove(path, wrm_path, args.noninteractive, args.quiet)?
        }
    } else {
        let mut cmd = Command::new("wrm");
        let e = cmd.error(ErrorKind::DisplayHelp, "Incorrect arguments");
        Err(e).map_err(|e| e.into()).map_err(WrmError)?
    }
    Ok(())
}

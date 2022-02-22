use std::path::{Path, PathBuf};

use clap::Parser;

/// Struct representing the arguments we want as part of the command line. This is easily extensible
/// if we want to add new features to the cli.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    file: PathBuf,
}

impl Args {
    pub fn file(&self) -> &Path {
        &self.file
    }
}

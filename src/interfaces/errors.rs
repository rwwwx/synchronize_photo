use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsErrors {
    #[error("Cannot read directory: {0}.")]
    CannotReadDirectory(PathBuf),
    #[error("Cannot read file: {0}.")]
    CannotReadFile(PathBuf),
    #[error("Cannot get dir entry.")]
    CannotGetDirEntry,
    #[error("Cannot parse date.")]
    DateParsingFailure,
}

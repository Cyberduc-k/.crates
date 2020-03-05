use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileInfo {
    pub name: PathBuf,
    pub source: String,
}

intern::interner!(FileInterner, FileInfo, FileId);

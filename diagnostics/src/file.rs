use std::path::PathBuf;

static mut FILE_INTERNER: FileInterner = FileInterner::new();

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct FileId(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileInfo {
    pub name: PathBuf,
    pub source: String,
}

impl FileId {
    pub fn new(name: impl Into<PathBuf>, source: impl Into<String>) -> Self {
        unsafe {
            FILE_INTERNER.intern(FileInfo {
                name: name.into(),
                source: source.into(),
            })
        }
    }
}

impl std::ops::Deref for FileId {
    type Target = FileInfo;

    fn deref(&self) -> &Self::Target {
        unsafe { &FILE_INTERNER.data[self.0] }
    }
}

struct FileInterner {
    data: Vec<FileInfo>,
}

impl FileInterner {
    const fn new() -> Self {
        FileInterner { data: Vec::new() }
    }

    fn intern(&mut self, value: FileInfo) -> FileId {
        if let Some(idx) = self.data.iter().position(|f| f.name == value.name) {
            FileId(idx)
        } else {
            self.data.push(value);

            FileId(self.data.len() - 1)
        }
    }
}

use std::path::{Path, PathBuf};

pub enum Action {
    Checked,
    Failed(PathBuf),
}

impl Action {
    pub fn is_checked(&self) -> bool {
        match self {
            Self::Checked => true,
            Self::Failed(_) => false,
        }
    }

    pub fn new_checked() -> Self {
        Self::Checked {}
    }

    pub fn new_failed(path: &Path) -> Self {
        Self::Failed {
            0: path.to_path_buf()
        }
    }

    pub fn failed(&self) -> Option<&Path> {
        match self {
            Self::Checked => None,
            Self::Failed(ref p) => Some(p)
        }
    }
}
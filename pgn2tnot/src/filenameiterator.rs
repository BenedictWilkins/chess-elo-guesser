use std::fs;
use std::path::{Path, PathBuf};

pub struct FileNameIterator {
    directory: PathBuf,
    entries: Option<fs::ReadDir>,
    extension: Option<String>,
}

impl FileNameIterator {
    pub fn new(directory: impl AsRef<Path>) -> Self {
        Self {
            directory: directory.as_ref().to_path_buf(),
            entries: None,
            extension: None, // 
        }
    }

    pub fn has_extension(mut self, extension: &str) -> Self {
        self.extension = Some(extension).map(|ext| ext.to_string());
        return self;
    }

    fn resolve_path(&self, path: &Path) -> PathBuf {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.directory.join(path)
        };

        absolute_path.canonicalize().unwrap_or(absolute_path)
    }

    fn has_matching_extension(&self, path: &Path) -> bool {
        if let Some(extension) = &self.extension {
            if let Some(file_extension) = path.extension() {
                //println!("{:?}, {:?}", file_extension, extension);
                return file_extension.to_str().unwrap() == extension;
            }
            return false;
        }
        true
    }
}

impl Iterator for FileNameIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.entries.is_none() {
                match fs::read_dir(&self.directory) {
                    Ok(entries) => self.entries = Some(entries),
                    Err(_) => return None,
                }
            }

            if let Some(entries) = &mut self.entries {
                if let Some(entry) = entries.next() {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() && self.has_matching_extension(&path) {
                            return Some(self.resolve_path(&path).to_string_lossy().into_owned());
                        }
                    }
                } else {
                    return None;
                }
            }
        }
    }
}

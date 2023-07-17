use std::path::{Path, PathBuf};
use std::ffi::OsStr;

pub struct FileNameChunker<I> where
    I: Iterator,
{
    iterator: I,
    count: u32,
    original_file_name: PathBuf,
}

impl<I> FileNameChunker<I>
where
    I: Iterator,
{
    pub fn new(iterator: I, original_file_name: impl AsRef<Path>) -> Self {
        FileNameChunker {
            iterator,
            count: 0,
            original_file_name: original_file_name.as_ref().to_path_buf(),
        }
    }
}

impl<I> Iterator for FileNameChunker<I>
where
    I: Iterator,
    I::Item: Sized,
{
    type Item = (I::Item, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iterator.next() {
            self.count += 1;
            let modified_name = format!("{}-({})", self.original_file_name.file_stem().unwrap().to_string_lossy(), self.count);
            let modified_path = change_file_name(&self.original_file_name, &modified_name);
            Some((item, modified_path))
        } else {
            None
        }
    }
}

fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}


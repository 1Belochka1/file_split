use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::fs as tokio_fs;
use lazy_static::lazy_static;

const PHOTO_TYPE: [&str; 8] = ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "nef", "svg"];

lazy_static! {
    static ref PHOTO_EXTENSIONS: HashSet<&'static str> = PHOTO_TYPE.iter().cloned().collect();
}

fn is_file_photo(file_path: &PathBuf) -> bool {
    file_path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext_str| PHOTO_EXTENSIONS.contains(ext_str))
        .unwrap_or(false)
}

pub fn filter_files(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files.iter().filter_map(|path| if is_file_photo(path) { Some(path) } else { None }).cloned().collect()
}

pub async fn get_path_files(path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    let mut dir_entries = tokio_fs::read_dir(path).await?;

    while let Some(dir_entry) = dir_entries.next_entry().await? {
        let file_type = dir_entry.file_type().await?;
        let path = dir_entry.path();

        if file_type.is_file() {
            files.push(path);
        } else if file_type.is_dir() {
            files.extend(Box::pin(get_path_files(&path)).await?);
        }
    }

    Ok(filter_files(files))
}
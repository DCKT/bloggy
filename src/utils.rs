use std::{fs, io, path::Path};

#[derive(serde::Serialize)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub content: String,
    pub path: String,
    pub tags: Vec<String>,
    #[serde(skip)]
    pub assets: Vec<ArticleAsset>,
}

#[derive(serde::Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub struct ArticleAsset {
    pub path: String,
    pub file_name: String,
}

pub const OUTPUT_FOLDER_PATH: &str = "./dist";

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

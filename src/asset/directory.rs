use std::fs;
use std::{collections::HashSet, path::PathBuf};

use super::SBType;
use super::AssetReader;

pub struct DirectoryReader {
    base_directory: PathBuf,
    metadata_file: Option<String>,
    metadata: SBType,
    assets_paths: HashSet<String>,
}

impl DirectoryReader {
    pub fn new(base_directory: &str) -> anyhow::Result<Self> {
        let mut directory_reader = Self {
            base_directory: PathBuf::from(base_directory),
            metadata_file: None,
            metadata: SBType::Nil,
            assets_paths: HashSet::new(),
        };

        for meta_name in ["/_metadata", "/.metadata"] {
            let meta_file = directory_reader.to_filesystem(meta_name)?;
            if fs::exists(&meta_file)? {
                directory_reader.metadata_file = Some(meta_name.to_string());
                directory_reader.metadata =
                    SBType::try_from(json::parse(&fs::read_to_string(&meta_file)?)?)?;
            }
        }

        directory_reader.assets_paths = directory_reader.scan_all("/")?;

        Ok(directory_reader)
    }

    pub fn to_filesystem(&self, path: &str) -> anyhow::Result<PathBuf> {
        if !path.starts_with('/') {
            anyhow::bail!("Asset path '{}' must be absolute", path)
        }
        let relative_path = path.trim_start_matches('/');
        Ok(self.base_directory.join(relative_path))
    }

    pub fn scan_all(&self, asset_directory: &str) -> anyhow::Result<HashSet<String>> {
        let fs_directory = self.to_filesystem(asset_directory)?;
        let mut output = HashSet::new();

        for entry in fs::read_dir(fs_directory)? {
            let entry = entry?;
            let file_name = entry.file_name().into_string().unwrap_or("".to_string());
            let asset_path = format!("{}{}", asset_directory, file_name);

            if entry.file_type()?.is_dir() {
                output.extend(self.scan_all(&format!("{}/", asset_path))?);
            } else {
                output.insert(asset_path.clone());
            }
        }

        Ok(output)
    }
}

impl AssetReader for DirectoryReader {
    fn exist(&self, path: &str) -> bool {
        if path.starts_with('/') {
            let relative_path = path.trim_start_matches('/');
            self.assets_paths.contains(relative_path)
        } else {
            false
        }
    }

    fn file(&mut self, path: &str) -> anyhow::Result<super::file::AssetFile> {
        if !self.exist(path) {
            anyhow::bail!("File is not exist")
        }
        let path = path.to_string();
        let file_path = self.to_filesystem(&path)?;

        let bytes = fs::read(file_path)?;

        Ok(super::file::AssetFile { path, bytes })
    }

    fn paths(&self) -> Vec<&String> {
        self.assets_paths.iter().collect()
    }

    fn meta(&self, key: String) -> anyhow::Result<SBType> {
        match self.metadata {
            SBType::Object(ref map) => {
                if let Some(value) = map.get(&key) {
                    Ok(value.clone())
                } else {
                    anyhow::bail!("Key '{}' not found in metadata", key)
                }
            }
            _ => {
                anyhow::bail!("Metadata is not an object")
            }
        }
    }
}
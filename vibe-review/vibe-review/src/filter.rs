use std::path::Path;

use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use thiserror::Error;

use crate::git::DiffFile;

#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub respect_gitignore: bool,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            include_patterns: vec![
                String::from("*.rs"),
                String::from("Cargo.toml"),
                String::from("Cargo.lock"),
            ],
            exclude_patterns: vec![String::from("target/**"), String::from("*.generated.rs")],
            respect_gitignore: true,
        }
    }
}

#[derive(Debug, Error)]
pub enum FilterError {
    #[error("failed to build glob patterns: {0}")]
    InvalidPattern(String),
    #[error("failed to read gitignore: {0}")]
    Gitignore(String),
}

pub fn filter_files(
    files: Vec<DiffFile>,
    config: &FilterConfig,
) -> Result<Vec<DiffFile>, FilterError> {
    let include = build_glob_set(&config.include_patterns)?;
    let exclude = build_glob_set(&config.exclude_patterns)?;
    let gitignore = if config.respect_gitignore {
        Some(load_gitignore()?)
    } else {
        None
    };

    let mut filtered = Vec::new();
    for file in files {
        if !include.is_match(&file.path) {
            continue;
        }
        if exclude.is_match(&file.path) {
            continue;
        }
        if let Some(gitignore) = &gitignore
            && gitignore.matched(&file.path, false).is_ignore()
        {
            continue;
        }
        filtered.push(file);
    }

    Ok(filtered)
}

fn build_glob_set(patterns: &[String]) -> Result<GlobSet, FilterError> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob =
            Glob::new(pattern).map_err(|err| FilterError::InvalidPattern(err.to_string()))?;
        builder.add(glob);
    }
    builder
        .build()
        .map_err(|err| FilterError::InvalidPattern(err.to_string()))
}

fn load_gitignore() -> Result<Gitignore, FilterError> {
    let root = Path::new(".");
    let mut builder = GitignoreBuilder::new(root);
    builder.add(".gitignore");
    builder
        .build()
        .map_err(|err| FilterError::Gitignore(err.to_string()))
}

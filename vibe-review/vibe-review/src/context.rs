use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::git::DiffResult;

#[derive(Debug)]
pub struct ReviewContext {
    pub style_guide: Option<String>,
    pub project_readme: Option<String>,
    pub diff: DiffResult,
}

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("failed to read style guide: {0}")]
    StyleGuide(String),
    #[error("failed to read README: {0}")]
    Readme(String),
    #[error("path traversal detected: {0} is outside project root")]
    PathTraversal(String),
}

pub fn build_context(
    diff: DiffResult,
    style_dir: &Path,
    project_root: &Path,
) -> Result<ReviewContext, ContextError> {
    let style_guide = load_style_guide(style_dir, project_root).ok();
    let project_readme = load_readme(project_root).ok();

    Ok(ReviewContext {
        style_guide,
        project_readme,
        diff,
    })
}

/// 경로가 허용된 루트 디렉토리 내부에 있는지 검증합니다.
fn validate_path_within_root(path: &Path, root: &Path) -> Result<(), ContextError> {
    let canonical_path = path
        .canonicalize()
        .map_err(|e| ContextError::PathTraversal(e.to_string()))?;
    let canonical_root = root
        .canonicalize()
        .map_err(|e| ContextError::PathTraversal(e.to_string()))?;

    if !canonical_path.starts_with(&canonical_root) {
        return Err(ContextError::PathTraversal(
            canonical_path.display().to_string(),
        ));
    }
    Ok(())
}

fn load_style_guide(style_dir: &Path, project_root: &Path) -> Result<String, ContextError> {
    let path = style_dir.join("rust.md");

    // 파일이 존재하는 경우에만 경로 검증 (존재하지 않으면 읽기 실패로 처리)
    if path.exists() {
        validate_path_within_root(&path, project_root)?;
    }

    fs::read_to_string(&path).map_err(|err| ContextError::StyleGuide(err.to_string()))
}

fn load_readme(project_root: &Path) -> Result<String, ContextError> {
    let candidates = ["README.md", "readme.md"];
    for name in candidates {
        let path = project_root.join(name);
        if path.exists() {
            validate_path_within_root(&path, project_root)?;
            return fs::read_to_string(&path)
                .map_err(|err| ContextError::Readme(err.to_string()));
        }
    }
    Err(ContextError::Readme(String::from("README not found")))
}

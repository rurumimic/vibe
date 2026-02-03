use std::process::Command;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Toml,
    Markdown,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffFile {
    pub path: String,
    pub status: FileStatus,
    pub diff: String,
    pub language: Language,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffStats {
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffResult {
    pub files: Vec<DiffFile>,
    pub stats: DiffStats,
}

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct DiffConfig {
    pub project: PathBuf,
    pub commit: Option<String>,
    pub base: Option<String>,
    pub head: Option<String>,
    pub staged: bool,
}

#[derive(Debug, Error)]
pub enum DiffError {
    #[error("git diff failed: {0}")]
    CommandFailed(String),
    #[error("diff output was not valid utf-8")]
    InvalidOutput,
    #[error("no diff content detected")]
    EmptyDiff,
}

#[derive(Debug)]
struct NameStatus {
    status: String,
    path: String,
}

/// 지정된 경로에서 git 저장소 루트를 찾습니다.
fn find_git_root(start_path: &PathBuf) -> Result<PathBuf, DiffError> {
    let output = Command::new("git")
        .current_dir(start_path)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|err| DiffError::CommandFailed(err.to_string()))?;

    if !output.status.success() {
        return Err(DiffError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let root = String::from_utf8(output.stdout)
        .map_err(|_| DiffError::InvalidOutput)?
        .trim()
        .to_string();

    Ok(PathBuf::from(root))
}

pub fn extract_diff(config: &DiffConfig) -> Result<DiffResult, DiffError> {
    // 지정된 project 경로에서 git 루트를 자동으로 찾음
    let git_root = find_git_root(&config.project)?;
    let range = build_range(config);
    let diff_output = run_git_diff(&git_root, &range, config.staged)?;
    if diff_output.trim().is_empty() {
        return Err(DiffError::EmptyDiff);
    }

    let files = parse_name_status(&git_root, &range, config.staged)?;
    let mut entries = Vec::new();
    for file in files {
        let diff = run_git_file_diff(&git_root, &range, config.staged, &file.path)?;
        entries.push(DiffFile {
            path: file.path.clone(),
            status: map_status(&file.status),
            diff,
            language: detect_language(&file.path),
        });
    }

    let stats = parse_stats(&diff_output);

    Ok(DiffResult {
        files: entries,
        stats,
    })
}

fn build_range(config: &DiffConfig) -> String {
    if let Some(commit) = &config.commit {
        return format!("{commit}^!");
    }

    // staged 모드에서는 HEAD와 비교해야 새 파일도 표시됨
    if config.staged {
        return String::from("HEAD");
    }

    match (&config.base, &config.head) {
        (Some(base), Some(head)) => format!("{base}..{head}"),
        (Some(base), None) => format!("{base}..HEAD"),
        (None, Some(head)) if head != "HEAD" => format!("{head}^!"),
        _ => String::from("HEAD^!"),
    }
}

fn run_git_diff(project: &PathBuf, range: &str, staged: bool) -> Result<String, DiffError> {
    let mut command = Command::new("git");
    command.current_dir(project);
    command.arg("diff").arg("--unified=3");
    if staged {
        command.arg("--cached");
    }
    if !range.is_empty() {
        command.arg(range);
    }

    let output = command
        .output()
        .map_err(|err| DiffError::CommandFailed(err.to_string()))?;
    if !output.status.success() {
        return Err(DiffError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    String::from_utf8(output.stdout).map_err(|_| DiffError::InvalidOutput)
}

fn run_git_file_diff(
    project: &PathBuf,
    range: &str,
    staged: bool,
    path: &str,
) -> Result<String, DiffError> {
    let mut command = Command::new("git");
    command.current_dir(project);
    command.arg("diff").arg("--unified=3");
    if staged {
        command.arg("--cached");
    }
    if !range.is_empty() {
        command.arg(range);
    }
    // range 이후에 -- 구분자를 넣어야 경로와 revision을 구분할 수 있음
    command.arg("--").arg(path);

    let output = command
        .output()
        .map_err(|err| DiffError::CommandFailed(err.to_string()))?;
    if !output.status.success() {
        return Err(DiffError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    String::from_utf8(output.stdout).map_err(|_| DiffError::InvalidOutput)
}

fn parse_name_status(
    project: &PathBuf,
    range: &str,
    staged: bool,
) -> Result<Vec<NameStatus>, DiffError> {
    let mut command = Command::new("git");
    command.current_dir(project);
    command.arg("diff").arg("--name-status");
    if staged {
        command.arg("--cached");
    }
    if !range.is_empty() {
        command.arg(range);
    }

    let output = command
        .output()
        .map_err(|err| DiffError::CommandFailed(err.to_string()))?;
    if !output.status.success() {
        return Err(DiffError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let stdout = String::from_utf8(output.stdout).map_err(|_| DiffError::InvalidOutput)?;
    let mut results = Vec::new();
    for line in stdout.lines() {
        let mut parts = line.split_whitespace();
        let status = match parts.next() {
            Some(status) => status,
            None => continue,
        };
        let path = parts.last().unwrap_or_default();
        if path.is_empty() {
            continue;
        }
        results.push(NameStatus {
            status: status.to_string(),
            path: path.to_string(),
        });
    }
    Ok(results)
}

fn parse_stats(diff: &str) -> DiffStats {
    let mut stats = DiffStats {
        files_changed: 0,
        insertions: 0,
        deletions: 0,
    };

    for line in diff.lines() {
        if !line.starts_with(' ') || !line.contains("files changed") {
            continue;
        }
        let tokens: Vec<&str> = line.split(',').collect();
        if let Some(files) = tokens.first() {
            stats.files_changed = parse_number(files);
        }
        if let Some(insertions) = tokens.get(1) {
            stats.insertions = parse_number(insertions);
        }
        if let Some(deletions) = tokens.get(2) {
            stats.deletions = parse_number(deletions);
        }
    }

    stats
}

fn parse_number(segment: &str) -> usize {
    segment
        .split_whitespace()
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}

fn map_status(status: &str) -> FileStatus {
    match status.chars().next() {
        Some('A') => FileStatus::Added,
        Some('D') => FileStatus::Deleted,
        Some('R') => FileStatus::Renamed,
        _ => FileStatus::Modified,
    }
}

fn detect_language(path: &str) -> Language {
    if path.ends_with(".rs") {
        return Language::Rust;
    }
    if path.ends_with("Cargo.toml") || path.ends_with("Cargo.lock") {
        return Language::Toml;
    }
    if path.ends_with(".md") {
        return Language::Markdown;
    }
    Language::Other
}

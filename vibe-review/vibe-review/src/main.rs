mod client;
mod context;
mod filter;
mod git;
mod output;
mod prompt;

use std::path::PathBuf;

use clap::Parser;
use thiserror::Error;

use crate::client::ClientError;
use crate::context::ContextError;
use crate::filter::FilterError;
use crate::git::DiffError;
use crate::output::OutputError;

#[derive(Debug, Parser)]
#[command(name = "vibe-review", version, about = "Rust code review bot for Vibe")]
struct Cli {
    /// Project root directory (git repository root)
    #[arg(short = 'p', long = "project", default_value = ".")]
    project: PathBuf,
    /// Specific commit hash
    #[arg(short = 'c', long = "commit")]
    commit: Option<String>,
    /// Base branch for comparison
    #[arg(short = 'b', long = "base")]
    base: Option<String>,
    /// Head branch for comparison
    #[arg(long = "head", default_value = "HEAD")]
    head: String,
    /// Staged changes only
    #[arg(short = 's', long = "staged")]
    staged: bool,
    /// Output file path
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
    /// Style guide directory
    #[arg(long = "style-dir", default_value = "docs/styles")]
    style_dir: PathBuf,
    /// Verbose logging
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
    /// Output prompt without API call
    #[arg(long = "dry-run")]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();
    if cli.verbose {
        eprintln!("Starting vibe-review");
    }

    if let Err(error) = run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

#[derive(Debug, Error)]
enum AppError {
    #[error("{0}")]
    Diff(#[from] DiffError),
    #[error("{0}")]
    Filter(#[from] FilterError),
    #[error("{0}")]
    Context(#[from] ContextError),
    #[error("{0}")]
    Client(#[from] ClientError),
    #[error("{0}")]
    Output(#[from] OutputError),
}

fn run(cli: Cli) -> Result<(), AppError> {
    let project_root = cli.project.canonicalize().unwrap_or(cli.project.clone());
    let diff_config = git::DiffConfig {
        project: project_root.clone(),
        commit: cli.commit.clone(),
        base: cli.base.clone(),
        head: Some(cli.head.clone()),
        staged: cli.staged,
    };
    let diff = git::extract_diff(&diff_config)?;

    let filter_config = filter::FilterConfig::default();
    let filtered = filter::filter_files(diff.files, &filter_config)?;
    let diff = git::DiffResult {
        files: filtered,
        stats: diff.stats,
    };

    let style_dir = project_root.join(&cli.style_dir);
    let context = context::build_context(diff, &style_dir, &project_root)?;
    let prompt = prompt::build_prompt(&context);

    if cli.dry_run {
        output::write_output(cli.output.as_deref(), &prompt)?;
        return Ok(());
    }

    let response = client::send_review_request(&prompt)?;
    output::write_output(cli.output.as_deref(), &response)?;
    Ok(())
}

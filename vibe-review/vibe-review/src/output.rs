use std::fs;
use std::io::{self, Write};
use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("failed to write output: {0}")]
    Io(String),
}

pub fn write_output(path: Option<&Path>, content: &str) -> Result<(), OutputError> {
    match path {
        Some(path) => fs::write(path, content).map_err(|err| OutputError::Io(err.to_string())),
        None => {
            let mut stdout = io::stdout();
            stdout
                .write_all(content.as_bytes())
                .and_then(|_| stdout.flush())
                .map_err(|err| OutputError::Io(err.to_string()))
        }
    }
}

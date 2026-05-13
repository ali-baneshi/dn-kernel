use anyhow::{Context, Result};
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn health() -> Result<&'static str> {
    Ok("ok")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptions {
    pub max_depth: usize,
    pub max_files: usize,
    pub max_bytes_total: u64,
    pub max_report_bytes: usize,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            max_depth: 12,
            max_files: 20_000,
            max_bytes_total: 256 * 1024 * 1024,
            max_report_bytes: 8 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub size_bytes: u64,
    pub extension: Option<String>,
    pub binary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub root: String,
    pub files: Vec<FileEntry>,
    pub total_files: usize,
    pub total_bytes: u64,
    pub truncated: bool,
    pub errors: Vec<String>,
}

pub fn scan_repository(root: impl AsRef<Path>, options: ScanOptions) -> Result<ScanReport> {
    let root = normalize_root(root.as_ref())?;

    let mut report = ScanReport {
        root: root.to_string_lossy().to_string(),
        files: Vec::new(),
        total_files: 0,
        total_bytes: 0,
        truncated: false,
        errors: Vec::new(),
    };

    let mut builder = WalkBuilder::new(&root);
    builder
        .standard_filters(true)
        .hidden(false)
        .follow_links(false)
        .max_depth(Some(options.max_depth));

    for item in builder.build() {
        let entry = match item {
            Ok(entry) => entry,
            Err(err) => {
                report.errors.push(err.to_string());
                continue;
            }
        };

        let path = entry.path();

        if path == root {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                report
                    .errors
                    .push(format!("metadata failed for {}: {err}", path.display()));
                continue;
            }
        };

        if !metadata.is_file() {
            continue;
        }

        if report.files.len() >= options.max_files {
            report.truncated = true;
            break;
        }

        let size = metadata.len();

        if report.total_bytes.saturating_add(size) > options.max_bytes_total {
            report.truncated = true;
            break;
        }

        let binary = match is_binary_file(path) {
            Ok(value) => value,
            Err(err) => {
                report.errors.push(format!(
                    "binary detection failed for {}: {err}",
                    path.display()
                ));
                true
            }
        };

        let relative = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .map(ToOwned::to_owned);

        report.total_bytes = report.total_bytes.saturating_add(size);
        report.total_files += 1;
        report.files.push(FileEntry {
            path: relative,
            size_bytes: size,
            extension,
            binary,
        });
    }

    enforce_report_size_limit(&mut report, options.max_report_bytes)?;

    Ok(report)
}

fn normalize_root(path: &Path) -> Result<PathBuf> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("failed to canonicalize path: {}", path.display()))?;

    if !canonical.is_dir() {
        anyhow::bail!("scan root is not a directory: {}", canonical.display());
    }

    Ok(canonical)
}

fn is_binary_file(path: &Path) -> Result<bool> {
    let mut file =
        File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
    let mut buffer = [0_u8; 8192];
    let read = file
        .read(&mut buffer)
        .with_context(|| format!("failed to read {}", path.display()))?;

    if read == 0 {
        return Ok(false);
    }

    Ok(buffer[..read].contains(&0))
}

fn enforce_report_size_limit(report: &mut ScanReport, max_report_bytes: usize) -> Result<()> {
    while serde_json::to_vec(report)?.len() > max_report_bytes && !report.files.is_empty() {
        report.files.pop();
        report.truncated = true;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_are_sane() {
        let options = ScanOptions::default();
        assert!(options.max_depth > 0);
        assert!(options.max_files > 0);
        assert!(options.max_bytes_total > 0);
        assert!(options.max_report_bytes > 0);
    }
}

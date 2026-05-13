pub mod worker;

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::worker::registry::WorkerRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptions {
    pub include_hidden: bool,
    pub max_file_size_bytes: u64,
    pub include_content: bool,
    pub max_file_read_bytes: usize,
    pub enable_worker_python: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            include_hidden: false,
            max_file_size_bytes: 1024 * 1024,
            include_content: false,
            max_file_read_bytes: 32 * 1024,
            enable_worker_python: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: String,
    pub rule: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
    pub findings: Vec<Finding>,
    pub content_preview: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerAnalyzeFileParams {
    pub path: String,
    pub language: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerRequest {
    pub protocol_version: String,
    pub request_id: String,
    pub method: String,
    pub params: WorkerAnalyzeFileParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerResponse {
    pub protocol_version: String,
    pub request_id: String,
    pub status: String,
    #[serde(default)]
    pub findings: Vec<Finding>,
    #[serde(default)]
    pub error: Option<String>,
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn is_text_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => matches!(
            ext,
            "rs" | "toml"
                | "md"
                | "txt"
                | "json"
                | "yaml"
                | "yml"
                | "py"
                | "js"
                | "ts"
                | "tsx"
                | "jsx"
                | "java"
                | "c"
                | "h"
                | "cpp"
                | "hpp"
                | "go"
                | "rb"
                | "php"
                | "sh"
                | "html"
                | "css"
                | "sql"
        ),
        None => false,
    }
}

fn detect_language(path: &Path) -> Option<String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("py") => Some("python".to_string()),
        Some("rs") => Some("rust".to_string()),
        Some("js") => Some("javascript".to_string()),
        Some("ts") => Some("typescript".to_string()),
        Some("tsx") => Some("typescript".to_string()),
        Some("jsx") => Some("javascript".to_string()),
        Some("json") => Some("json".to_string()),
        Some("toml") => Some("toml".to_string()),
        Some("md") => Some("markdown".to_string()),
        _ => None,
    }
}

fn read_text_preview(path: &Path, max_bytes: usize) -> Result<String, String> {
    let mut file = fs::File::open(path).map_err(|e| format!("open {}: {}", path.display(), e))?;
    let mut buffer = vec![0u8; max_bytes];
    let bytes_read = file
        .read(&mut buffer)
        .map_err(|e| format!("read {}: {}", path.display(), e))?;
    buffer.truncate(bytes_read);
    String::from_utf8(buffer).map_err(|e| format!("utf8 {}: {}", path.display(), e))
}

fn run_rules(content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let lower = content.to_lowercase();

    if content.contains("TODO") {
        findings.push(Finding {
            severity: "info".to_string(),
            rule: "todo-comment".to_string(),
            message: "TODO marker found in file content".to_string(),
        });
    }

    if content.contains("unsafe") {
        findings.push(Finding {
            severity: "warning".to_string(),
            rule: "unsafe-usage".to_string(),
            message: "Possible unsafe usage detected".to_string(),
        });
    }

    if lower.contains("password") || lower.contains("secret") {
        findings.push(Finding {
            severity: "high".to_string(),
            rule: "possible-secret".to_string(),
            message: "Possible secret-related text detected".to_string(),
        });
    }

    findings
}

fn run_python_worker(path: &Path, content: &str) -> Result<Vec<Finding>, String> {
    let request = WorkerRequest {
        protocol_version: "0.1.0".to_string(),
        request_id: format!("scan-{}", path.display()),
        method: "analyze_file".to_string(),
        params: WorkerAnalyzeFileParams {
            path: path.display().to_string(),
            language: detect_language(path),
            content: content.to_string(),
        },
    };

    let request_json =
        serde_json::to_vec(&request).map_err(|e| format!("serialize worker request: {}", e))?;

    let registry = WorkerRegistry::new();

    let config = registry
        .get("python")
        .ok_or_else(|| "python worker not registered".to_string())?;

    let mut child = Command::new(&config.command)
        .args(&config.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn python worker: {}", e))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "python worker stdin unavailable".to_string())?;
        use std::io::Write;
        stdin
            .write_all(&request_json)
            .map_err(|e| format!("write worker stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("wait for python worker: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("python worker failed: {}", stderr.trim()));
    }

    let response: WorkerResponse = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("parse worker response: {}", e))?;

    if response.status != "ok" {
        return Err(response
            .error
            .unwrap_or_else(|| "python worker returned non-ok status".to_string()));
    }

    Ok(response.findings)
}

fn scan_dir(
    root: &Path,
    current: &Path,
    options: &ScanOptions,
    files: &mut Vec<FileEntry>,
    total_bytes: &mut u64,
    truncated: &mut bool,
    errors: &mut Vec<String>,
) {
    let entries = match fs::read_dir(current) {
        Ok(entries) => entries,
        Err(err) => {
            errors.push(format!("read_dir {}: {}", current.display(), err));
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                errors.push(format!("dir_entry {}: {}", current.display(), err));
                continue;
            }
        };

        let path = entry.path();

        if !options.include_hidden && is_hidden(&path) {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                errors.push(format!("metadata {}: {}", path.display(), err));
                continue;
            }
        };

        if metadata.is_dir() {
            scan_dir(root, &path, options, files, total_bytes, truncated, errors);
            continue;
        }

        if !metadata.is_file() {
            continue;
        }

        let size = metadata.len();
        *total_bytes += size;

        if size > options.max_file_size_bytes {
            *truncated = true;
            continue;
        }

        let rel_path = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();

        let mut findings = Vec::new();
        let mut content_preview = None;

        if is_text_file(&path) {
            match read_text_preview(&path, options.max_file_read_bytes) {
                Ok(content) => {
                    findings.extend(run_rules(&content));

                    if options.enable_worker_python
                        && detect_language(&path).as_deref() == Some("python")
                    {
                        match run_python_worker(&path, &content) {
                            Ok(worker_findings) => findings.extend(worker_findings),
                            Err(err) => errors.push(format!("worker {}: {}", path.display(), err)),
                        }
                    }

                    if options.include_content {
                        content_preview = Some(content);
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        files.push(FileEntry {
            path: rel_path,
            size,
            findings,
            content_preview,
        });
    }
}

pub fn scan_repository(root: impl AsRef<Path>, options: &ScanOptions) -> ScanReport {
    let root = root.as_ref();
    let root_path: PathBuf = root.to_path_buf();
    let mut files = Vec::new();
    let mut total_bytes = 0u64;
    let mut truncated = false;
    let mut errors = Vec::new();

    scan_dir(
        &root_path,
        &root_path,
        options,
        &mut files,
        &mut total_bytes,
        &mut truncated,
        &mut errors,
    );

    let total_files = files.len();

    ScanReport {
        root: root_path.to_string_lossy().to_string(),
        files,
        total_files,
        total_bytes,
        truncated,
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_rules_detects_todo_unsafe_and_secret() {
        let content = "TODO: fix this\nunsafe { }\npassword=123\nsecret token\n";
        let findings = run_rules(content);

        assert!(findings.iter().any(|f| f.rule == "todo-comment"));
        assert!(findings.iter().any(|f| f.rule == "unsafe-usage"));
        assert!(findings.iter().any(|f| f.rule == "possible-secret"));
    }

    #[test]
    fn detect_language_recognizes_python() {
        let lang = detect_language(Path::new("example.py"));
        assert_eq!(lang.as_deref(), Some("python"));
    }

    #[test]
    fn detect_language_returns_none_for_unknown_extension() {
        let lang = detect_language(Path::new("example.unknown"));
        assert_eq!(lang, None);
    }

    #[test]
    fn is_text_file_recognizes_python_file() {
        assert!(is_text_file(Path::new("script.py")));
    }

    #[test]
    fn is_text_file_rejects_unknown_extension() {
        assert!(!is_text_file(Path::new("archive.bin")));
    }
}

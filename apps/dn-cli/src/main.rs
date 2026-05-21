use std::io::Write;
use std::path::Path;
use std::process;

use clap::{Parser, Subcommand};
use dn_runtime::{available_profiles, scan_repository, OutputFormat, ScanOptions};

#[derive(Parser, Debug)]
#[command(name = "dn-cli")]
#[command(version)]
#[command(about = "Terminal code review and repository audit assistant")]
#[command(
    long_about = "dn-cli helps you quickly audit unfamiliar, AI-generated, copied, or legacy code.\n\
It scans files through a profile-defined policy and outputs stable JSON or readable Markdown.\n\n\
Examples:\n\
  dn-cli scan . --profile quick\n\
  dn-cli scan . --profile security --json --hidden\n\
  dn-cli review . --profile architecture --markdown\n\
  dn-cli scan . --profile my-security --content\n\
  dn-cli scan . --profile quick --python-worker"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan a repository path and emit a repository review.
    Scan {
        /// Path to scan (repository root or folder)
        path: String,

        #[arg(long, default_value = "quick")]
        /// Profile name or .dn/profiles/<name>.toml/.yml/.yaml entry
        profile: String,

        /// Print machine-readable JSON output
        #[arg(long, conflicts_with = "markdown")]
        json: bool,

        /// Print markdown output
        #[arg(long, conflicts_with = "json")]
        markdown: bool,

        /// Include up to 1KB content preview for scanned files in JSON/Markdown output
        #[arg(long)]
        content: bool,

        /// Include dotfiles and dot-directories while scanning
        #[arg(long)]
        hidden: bool,

        /// Enable Python worker in addition to profile configuration
        #[arg(long)]
        python_worker: bool,
    },
    /// Review is a first-class alias for scan.
    Review {
        /// Path to review (repository root or folder)
        path: String,

        #[arg(long, default_value = "quick")]
        profile: String,

        /// Print machine-readable JSON output
        #[arg(long, conflicts_with = "markdown")]
        json: bool,

        /// Print markdown output
        #[arg(long, alias = "md", conflicts_with = "json")]
        markdown: bool,

        #[arg(long)]
        content: bool,

        #[arg(long)]
        hidden: bool,

        #[arg(long)]
        python_worker: bool,
    },
}

fn render_markdown(report: &dn_runtime::ScanReport) -> String {
    let mut output = String::new();
    let mut findings_count = 0usize;
    let mut files_with_no_findings = 0usize;
    for file in &report.files {
        findings_count += file.findings.len();
        if file.findings.is_empty() {
            files_with_no_findings += 1;
        }
    }

    let mut files_with_findings = 0usize;

    output.push_str(&format!(
        "# dn-kernel Review Report\n\nProfile: `{}`\n\n",
        report.profile
    ));
    output.push_str(&format!("- Root: `{}`\n", report.root));
    output.push_str(&format!("- Profile source: `{}`\n", report.profile_source));
    output.push_str(&format!("- Provider: `{}`\n", report.provider));
    output.push_str(&format!("- Worker: `{}`\n", report.worker));
    output.push_str(&format!(
        "- Files discovered: {}\n",
        report.files_discovered
    ));
    output.push_str(&format!("- Files analyzed: {}\n", report.files_scanned));
    output.push_str(&format!(
        "- Files returned in report: {}\n",
        report.files_selected
    ));
    output.push_str(&format!("- Skipped files: {}\n", report.files_skipped));
    output.push_str(&format!(
        "- Skipped large files: {}\n",
        report.skipped_large_files
    ));
    output.push_str(&format!("- Total bytes read: {}\n", report.total_bytes));
    output.push_str(&format!(
        "- Truncated by scan limits: {}\n",
        report.truncated
    ));
    output.push_str(&format!("- Duration: {}ms\n\n", report.duration_ms));
    output.push_str(&format!(
        "- Findings: info={} low={} medium={} high={} critical={}\n\n",
        report.severity_breakdown.info,
        report.severity_breakdown.low,
        report.severity_breakdown.medium,
        report.severity_breakdown.high,
        report.severity_breakdown.critical
    ));
    output.push_str(&format!(
        "## Summary\n\n- Total findings: {}\n- Files with findings: {}\n- Files reported without findings: {}\n\n",
        findings_count,
        report.files.len().saturating_sub(files_with_no_findings),
        files_with_no_findings
    ));

    output.push_str("## Top findings\n\n");
    for file in &report.files {
        if file.findings.is_empty() {
            continue;
        }
        files_with_findings += 1;
        output.push_str(&format!("### `{}`\n", file.path));
        for finding in &file.findings {
            output.push_str(&format!(
                "- **{}** [{}] {}: {}\n",
                finding.severity,
                finding.rule,
                finding.source.clone().unwrap_or_else(|| "scan".to_string()),
                finding.message
            ));
        }
        output.push('\n');
    }
    if files_with_findings == 0 {
        output.push_str("*No findings were reported for the current profile.*\n\n");
    }

    if !report.errors.is_empty() {
        output.push_str("## Errors\n\n");
        output.push_str(&format!("- Count: {}\n", report.errors.len()));
        for error in &report.errors {
            output.push_str(&format!("- {}\n", error));
        }
    }

    output
}

fn print_report(report: dn_runtime::ScanReport, want_json: bool, want_markdown: bool) {
    if want_json {
        match serde_json::to_string_pretty(&report) {
            Ok(payload) => println!("{payload}"),
            Err(err) => {
                eprintln!("error: failed to serialize report: {err}");
                process::exit(1);
            }
        }
        return;
    }

    if want_markdown {
        print!("{}", render_markdown(&report));
        return;
    }

    let mut findings_count = 0usize;
    for file in &report.files {
        findings_count += file.findings.len();
    }

    println!("root={}", report.root);
    println!("profile={}", report.profile);
    println!("profile_source={}", report.profile_source);
    println!("provider={}", report.provider);
    println!("files_discovered={}", report.files_discovered);
    println!("files_scanned={}", report.files_scanned);
    println!("files_selected={}", report.files_selected);
    println!("files_skipped={}", report.files_skipped);
    println!("skipped_large_files={}", report.skipped_large_files);
    println!("total_files={}", report.total_files);
    println!("bytes={}", report.total_bytes);
    println!("truncated={}", report.truncated);
    println!("duration_ms={}", report.duration_ms);
    println!("findings={}", findings_count);
    println!(
        "severity=info:{} low:{} medium:{} high:{} critical:{}",
        report.severity_breakdown.info,
        report.severity_breakdown.low,
        report.severity_breakdown.medium,
        report.severity_breakdown.high,
        report.severity_breakdown.critical
    );
    if !report.errors.is_empty() {
        println!("errors:");
        for error in &report.errors {
            println!("  - {}", error);
        }
    }
}

fn run_scan(
    path: String,
    profile: String,
    output_path: String,
    content: bool,
    hidden: bool,
    python_worker: bool,
    json: bool,
    markdown: bool,
) {
    let options = ScanOptions {
        profile_name: profile,
        include_hidden: hidden,
        include_content: content,
        python_worker,
        max_files: if python_worker { 100_000 } else { 10_000 },
        format: if json {
            OutputFormat::Json
        } else if markdown {
            OutputFormat::Markdown
        } else {
            OutputFormat::Text
        },
    };

    let report = match scan_repository(&path, &options) {
        Ok(report) => report,
        Err(err) => {
            let has_unknown_profile = err
                .chain()
                .any(|cause| cause.to_string().contains("unknown profile"));

            let mut first = true;
            for cause in err.chain() {
                if first {
                    eprintln!("error: {cause}");
                    first = false;
                } else {
                    eprintln!("  caused by: {cause}");
                }
            }
            if has_unknown_profile {
                let known = maybe_suggest_profile(&output_path);
                if !known.is_empty() {
                    eprintln!("hint: available profiles: {}", known.join(", "));
                    eprintln!("hint: local profiles are loaded from <scan root>/.dn/profiles");
                }
            }
            process::exit(1);
        }
    };
    print_report(report, json, markdown);
}

fn maybe_suggest_profile(path: impl AsRef<Path>) -> Vec<String> {
    available_profiles(path.as_ref())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            path,
            profile,
            json,
            markdown,
            content,
            hidden,
            python_worker,
        }
        | Commands::Review {
            path,
            profile,
            json,
            markdown,
            content,
            hidden,
            python_worker,
        } => {
            let _ = std::io::stdout().lock().flush();
            let root_for_hint = path.clone();
            run_scan(
                path,
                profile,
                root_for_hint,
                content,
                hidden,
                python_worker,
                json,
                markdown,
            );
        }
    }
}

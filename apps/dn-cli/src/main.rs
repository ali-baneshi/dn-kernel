use clap::{Parser, Subcommand};
use dn_runtime::{scan_repository, ScanOptions};

#[derive(Parser, Debug)]
#[command(name = "dn-cli")]
#[command(version)]
#[command(about = "Developer-native repository scanner")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan {
        path: String,

        #[arg(long)]
        json: bool,

        #[arg(long)]
        hidden: bool,

        #[arg(long)]
        content: bool,

        #[arg(long, default_value_t = 1024 * 1024)]
        max_file_size_bytes: u64,

        #[arg(long, default_value_t = 32 * 1024)]
        max_file_read_bytes: usize,

        #[arg(long)]
        python_worker: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            path,
            json,
            hidden,
            content,
            max_file_size_bytes,
            max_file_read_bytes,
            python_worker,
        } => {
            let options = ScanOptions {
                include_hidden: hidden,
                max_file_size_bytes,
                include_content: content,
                max_file_read_bytes,
                enable_worker_python: python_worker,
            };

            let report = scan_repository(path, &options);

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).expect("serialize scan report")
                );
            } else {
                let findings_count: usize =
                    report.files.iter().map(|file| file.findings.len()).sum();

                println!("root={}", report.root);
                println!("files={}", report.total_files);
                println!("bytes={}", report.total_bytes);
                println!("truncated={}", report.truncated);
                println!("errors={}", report.errors.len());
                println!("findings={}", findings_count);

                if !report.errors.is_empty() {
                    println!("error_details:");
                    for error in &report.errors {
                        println!("  - {}", error);
                    }
                }
            }
        }
    }
}

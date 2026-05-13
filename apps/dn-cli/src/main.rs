use anyhow::Result;
use clap::{Parser, Subcommand};
use dn_runtime::{health, scan_repository, ScanOptions};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "dn-cli")]
#[command(version)]
#[command(about = "dn-kernel command line interface")]
struct Cli {
    #[arg(long, global = true, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Health,
    Scan {
        #[arg(default_value = ".")]
        path: String,

        #[arg(long)]
        json: bool,

        #[arg(long, default_value_t = 12)]
        max_depth: usize,

        #[arg(long, default_value_t = 20_000)]
        max_files: usize,

        #[arg(long, default_value_t = 268_435_456)]
        max_bytes_total: u64,

        #[arg(long, default_value_t = 8_388_608)]
        max_report_bytes: usize,
    },
    Version,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(&cli.log_level);

    match cli.command {
        Some(Commands::Health) => {
            let status = health()?;
            println!("status={status}");
        }
        Some(Commands::Scan {
            path,
            json,
            max_depth,
            max_files,
            max_bytes_total,
            max_report_bytes,
        }) => {
            let options = ScanOptions {
                max_depth,
                max_files,
                max_bytes_total,
                max_report_bytes,
            };

            let report = scan_repository(path, options)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("root={}", report.root);
                println!("files={}", report.total_files);
                println!("bytes={}", report.total_bytes);
                println!("truncated={}", report.truncated);
                println!("errors={}", report.errors.len());
            }
        }
        Some(Commands::Version) => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        None => {
            println!("dn-cli ready");
            println!("try:");
            println!("  dn-cli health");
            println!("  dn-cli scan .");
            println!("  dn-cli scan . --json");
        }
    }

    Ok(())
}

fn init_logging(level: &str) {
    let filter = EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("info"));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}

//! A simple cli clock-in/clock-out utility for CSV files.
//!
//! # Usage
//! ```
//! clocker [INPUT_FILE] [OUTPUT_FILE]
//! ```

use clap::{Parser, Subcommand};
use std::env;
use std::path::{Path, PathBuf};

use crate::error::ClockerError;
use crate::timelog::TimeLog;

mod error;
mod timelog;

/// Version of the app as defined in the Cargo.toml file
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Default path used for input and output files
const DEFAULT_PATH: &str = "~/timelog-test.csv";

/// Utility to expand tilde in string path
fn resolve_path(path: &str) -> PathBuf {
    PathBuf::from(shellexpand::tilde(path).to_string())
}

/// Input structure to hold parsed parameters from command line
#[derive(Parser, Debug)]
#[command(author, version = APP_VERSION, about = "A simple cli clock-in/clock-out utility for CSV files.", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = DEFAULT_PATH, global = true)]
    input_file: String,
    #[arg(short, long, global = true)]
    output_file: Option<String>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Log,
    Archive,
    NewMonth,
}

/// Entrypoint of the tool
fn main() {
    let cli = Cli::parse();

    let input_filename = resolve_path(&cli.input_file);
    let output_filename = cli
        .output_file
        .as_deref()
        .map(resolve_path)
        .unwrap_or(input_filename.clone());

    let result = match cli.command {
        Some(Command::Archive) => archive(&input_filename, &output_filename),
        Some(Command::NewMonth) => new_month(&input_filename, &output_filename),
        Some(Command::Log) | None => log(&input_filename, &output_filename),
    };

    if let Err(error) = result {
        println!("{}", error);
    }
}

fn log<P: AsRef<Path>>(input: P, output: P) -> Result<(), ClockerError> {
    let time_log = TimeLog::from_file(&input)?;
    let _ = time_log.update()?.persist(&output)?;
    Ok(())
}

fn archive<P: AsRef<Path>>(input: P, output: P) -> Result<(), ClockerError> {
    // 1. daily log
    let time_log = TimeLog::from_file(&input)?;
    // 2. move file to archive
    let _ = time_log.update()?.backup(&input)?;
    // 3. init new file with empty TimeLog
    TimeLog::empty().persist(&output)?;
    Ok(())
}

fn new_month<P: AsRef<Path>>(input: P, output: P) -> Result<(), ClockerError> {
    // 1. move file to archive
    let time_log = TimeLog::from_file(&input)?;
    let _ = time_log.backup(&input)?;
    // 2. daily log
    let _ = TimeLog::empty().update()?.persist(&output)?;
    Ok(())
}

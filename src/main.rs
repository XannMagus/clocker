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
const DEFAULT_PATH: &str = "~/timelog.csv";

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
    #[command(alias = "l")]
    Log,
    #[command(alias = "a")]
    Archive,
    #[command(alias = "n")]
    NewMonth,
    #[command(alias = "s")]
    Snapshot,
    #[command(alias = "v", visible_alias = "show")]
    View {
        #[command(subcommand)]
        kind: Option<ViewCommand>,
    },
}

#[derive(Debug, Subcommand)]
enum ViewCommand {
    All,
    Latest,
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
        Some(Command::Snapshot) => snapshot(&input_filename),
        Some(Command::Log) | None => log(&input_filename, &output_filename),
        Some(Command::View { kind }) => match kind {
            Some(ViewCommand::Latest) => show_latest(&input_filename),
            Some(ViewCommand::All) | None => show_all(&input_filename),
        },
    };

    if let Err(error) = result {
        println!("{}", error);
    }
}

fn log<P: AsRef<Path>>(input: P, output: P) -> Result<(), ClockerError> {
    TimeLog::from_file(&input)?.update()?.persist(&output)?;
    Ok(())
}

fn archive<P: AsRef<Path>>(input: P, output: P) -> Result<(), ClockerError> {
    // 1. daily log
    // 2. move file to archive
    TimeLog::from_file(&input)?.update()?.backup(&input)?;
    // 3. init new file with empty TimeLog
    TimeLog::empty().persist(&output)?;
    Ok(())
}

fn new_month<P: AsRef<Path>>(input: P, output: P) -> Result<(), ClockerError> {
    // 1. move file to archive
    TimeLog::from_file(&input)?.backup(&input)?;
    // 2. daily log
    TimeLog::empty().update()?.persist(&output)?;
    Ok(())
}

fn snapshot<P: AsRef<Path>>(input: P) -> Result<(), ClockerError> {
    TimeLog::from_file(&input)?.backup(&input)?;
    Ok(())
}

fn show_all<P: AsRef<Path>>(input: P) -> Result<(), ClockerError> {
    let time_log = TimeLog::from_file(&input)?;
    println!("{}", time_log);
    Ok(())
}

fn show_latest<P: AsRef<Path>>(input: P) -> Result<(), ClockerError> {
    let time_log = TimeLog::from_file(&input)?;
    println!(
        "{}",
        time_log
            .latest_entry()
            .map_or(String::new(), |e| e.to_string())
    );
    Ok(())
}

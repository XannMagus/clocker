//! A simple cli clock-in/clock-out utility for CSV files.
//!
//! # Usage
//! ```
//! clocker [INPUT_FILE] [OUTPUT_FILE]
//! ```

use std::{env};
use clap::Parser;

use crate::timelog::TimeLog;

mod timelog;

/// Version of the app as defined in the Cargo.toml file
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Default path used for input and output files
const DEFAULT_PATH: &str = "~/horaires.csv";

/// Utility to expand tilde in string path
fn resolve_path(path: &str) -> String {
    shellexpand::tilde(path).to_string()
}

/// Input structure to hold parsed parameters from command line
#[derive(Parser, Debug)]
#[command(author, version = APP_VERSION, about = "A simple cli clock-in/clock-out utility for CSV files.", long_about = None)]
struct Cli {
    #[arg(default_value = DEFAULT_PATH)]
    input_file: String,
    output_file: Option<String>,
}

/// Entrypoint of the tool
fn main() {
    let cli = Cli::parse();

    let input_filename = resolve_path(&cli.input_file);
    let output_filename = cli.output_file.as_deref().map(resolve_path).unwrap_or(input_filename.clone());

    let time_log = TimeLog::from_file(&input_filename).expect(&format!("Couldn't read {}", input_filename));
    let _ = time_log.update().persist(&output_filename).expect("Couldn't write file");
}

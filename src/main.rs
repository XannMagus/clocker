use std::{env};
use clap::Parser;

use crate::timelog::TimeLog;

mod timelog;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_PATH: &str = "~/horaires.csv";

fn resolve_path(path: &str) -> String {
    shellexpand::tilde(path).to_string()
}

#[derive(Parser, Debug)]
#[command(author, version = APP_VERSION, about = "A simple cli clock-in/clock-out utility for CSV files.", long_about = None)]
struct Cli {
    #[arg(default_value = DEFAULT_PATH)]
    input_file: String,
    output_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let input_filename = resolve_path(&cli.input_file);
    let output_filename = cli.output_file.as_deref().map(resolve_path).unwrap_or(input_filename.clone());

    let time_log = TimeLog::from_file(&input_filename).expect(&format!("Couldn't read {}", input_filename));
    let _ = time_log.update().persist(&output_filename).expect("Couldn't write file");
}

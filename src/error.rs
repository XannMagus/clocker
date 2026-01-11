use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClockerError {
    #[error("Shift already complete for today.")]
    ShiftComplete,
    #[error("Malformed lines in the input file:\n{}", format_errors(.0))]
    FileParseError(Vec<csv::Error>),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

fn format_errors(errors: &Vec<csv::Error>) -> String {
    errors
        .iter()
        .map(|e| format!("{}", e))
        .collect::<Vec<_>>()
        .join("\n")
}

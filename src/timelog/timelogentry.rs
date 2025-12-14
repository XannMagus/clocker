//! Separates the TimeLogEntry struct from the main TimeLog module
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime};

/// Holds information about a single line of the CSV file
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TimeLogEntry {
    pub(super) date: NaiveDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) start_am: Option<NaiveTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) end_am: Option<NaiveTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) start_pm: Option<NaiveTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) end_pm: Option<NaiveTime>,
}

impl TimeLogEntry {
    /// Creates a new TimeLogEntry, representing a brand new line in the file
    pub fn new(date: NaiveDate, start_am: NaiveTime) -> Self {
        Self {
            date,
            start_am: Some(start_am),
            end_am: None,
            start_pm: None,
            end_pm: None,
        }
    }

    /// Sets time for the end of morning
    pub fn set_end_am(&mut self, end_am: NaiveTime) {
        self.end_am = Some(end_am);
    }

    /// Sets time for the start of the afternoon
    pub fn set_start_pm(&mut self, start_pm: NaiveTime) {
        self.start_pm = Some(start_pm);
    }

    /// Sets time for the end of the afternoon
    pub fn set_end_pm(&mut self, end_pm: NaiveTime) {
        self.end_pm = Some(end_pm);
    }
}

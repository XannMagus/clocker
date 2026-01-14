//! Module that groups data structures and logic of the application
//!
//! This module contains the main structure used by the application
//! to hold the state and implements the main logic of the operations
//!
use chrono::{Local, NaiveDate, NaiveTime, Timelike};
use std::{fmt::Display, fs, path::Path};
use timelogentry::TimeLogEntry;

use crate::{error::ClockerError, timelog::timelogentry::DayState};

mod timelogentry;

/// Describes the possible actions to take based on the current state of the file
#[derive(Debug)]
enum UpdateAction {
    NewDay(NaiveDate, NaiveTime),
    FillSlot(NaiveTime),
}

/// Main state structure. Holds information about the time and the existing log entries
#[derive(Debug)]
pub struct TimeLog {
    entries: Vec<TimeLogEntry>,
    today: NaiveDate,
    current_time: NaiveTime,
}

impl TimeLog {
    /// Loads entries from the file at the given path.
    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Result<Self, ClockerError> {
        let filepath = filepath.as_ref();
        if !filepath.exists() {
            return Ok(Self::empty());
        }

        let file = fs::File::open(filepath)?;
        let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(file);

        let mut entries = Vec::new();
        let mut errors = Vec::new();
        for log in reader.deserialize() {
            match log {
                Ok(log) => entries.push(log),
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            }
        }

        if errors.is_empty() {
            Ok(Self::new(entries))
        } else {
            Err(ClockerError::FileParseError(errors).into())
        }
    }

    /// Creates a new empty TimeLog
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn backup<P: AsRef<Path>>(&self, filepath: P) -> Result<(), ClockerError> {
        let backup_path = filepath.as_ref().with_extension("bak");
        self.persist(&backup_path)
    }

    /// Writes the current entries to the given filepath.
    pub fn persist<P: AsRef<Path>>(&self, filepath: P) -> Result<(), ClockerError> {
        let filepath = filepath.as_ref();
        let file = fs::File::create(filepath)?;
        let mut writer = csv::WriterBuilder::new().flexible(true).from_writer(file);
        for entry in self.entries.iter() {
            writer.serialize(entry)?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Updates the current entries and returns the result as new TimeLog
    pub fn update(self) -> Result<Self, ClockerError> {
        let action = self.determine_action()?;
        Ok(self.apply_action(action))
    }

    /// Returns a new Timelog with only the latest entry
    pub fn only_latest(self) -> TimeLog {
        Self { 
            entries: self.entries.last().cloned().into_iter().collect(), 
            today: self.today, 
            current_time: self.current_time, 
        }
    }

    /// Creates a new TimeLog from the given entries. Time and date are set to the current datetime
    fn new(entries: Vec<TimeLogEntry>) -> Self {
        let now = Local::now();
        let today = now.date_naive();
        let current_time = now
            .time()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        Self {
            entries,
            today,
            current_time,
        }
    }

    /// Decides what action is appropriate based on the current state
    fn determine_action(&self) -> Result<UpdateAction, ClockerError> {
        match self.entries.last() {
            None => Ok(UpdateAction::NewDay(self.today, self.current_time)),
            Some(TimeLogEntry { date: d, .. }) if d != &self.today => {
                Ok(UpdateAction::NewDay(self.today, self.current_time))
            }
            Some(last_entry) => match last_entry.state {
                DayState::DayFinished(_, _, _, _) => Err(ClockerError::ShiftComplete),
                _ => Ok(UpdateAction::FillSlot(self.current_time)),
            },
        }
    }

    /// Applies the given action and returns the result as a new TimeLog
    fn apply_action(mut self, action: UpdateAction) -> Self {
        match action {
            UpdateAction::NewDay(date, time) => {
                self.entries.push(TimeLogEntry::new(date, time));
            }
            UpdateAction::FillSlot(time) => {
                if let Some(last_entry) = self.entries.last_mut() {
                    last_entry.transition(time);
                }
            }
        }
        self
    }
}

impl Display for TimeLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"\u{256D}{}\u{252C}{}\u{252C}{}\u{252C}{}\u{252C}{}\u{256E}\n", "\u{2500}".repeat(14), "\u{2500}".repeat(12), "\u{2500}".repeat(12), "\u{2500}".repeat(12), "\u{2500}".repeat(12))?;
        write!(
            f,
            "\u{2502}{:^14}\u{2502}{:^12}\u{2502}{:^12}\u{2502}{:^12}\u{2502}{:^12}\u{2502}\n",
            "Date", "Start AM", "End AM", "Start PM", "End PM"
        )?;
        for entry in self.entries.iter() {
            write!(f,"\u{251C}{}\u{253C}{}\u{253C}{}\u{253C}{}\u{253C}{}\u{2524}\n", "\u{2500}".repeat(14), "\u{2500}".repeat(12), "\u{2500}".repeat(12), "\u{2500}".repeat(12), "\u{2500}".repeat(12))?;
            write!(f, "{}\n", entry)?;
        }
        write!(f,"\u{2570}{}\u{2534}{}\u{2534}{}\u{2534}{}\u{2534}{}\u{256F}\n", "\u{2500}".repeat(14), "\u{2500}".repeat(12), "\u{2500}".repeat(12), "\u{2500}".repeat(12), "\u{2500}".repeat(12))?;
        Ok(())
    }
}

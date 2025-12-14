//! Module that groups data structures and logic of the application
//!
//! This module contains the main structure used by the application
//! to hold the state and implements the main logic of the operations
//!
use chrono::{Local, NaiveDate, NaiveTime, Timelike};
use std::{error::Error, fs, io};
use timelogentry::TimeLogEntry;

mod timelogentry;

/// Holds the different legal times of day to log
#[derive(Debug)]
enum TimeOfDay {
    EndAM,
    StartPM,
    EndPM,
}

/// Describes the possible actions to take based on the current state of the file
#[derive(Debug)]
enum UpdateAction {
    NewDay(NaiveDate, NaiveTime),
    FillSlot(TimeOfDay, NaiveTime),
    NoChange,
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
    pub fn from_file(filepath: &String) -> io::Result<Self> {
        if !fs::metadata(&filepath).is_ok() {
            eprintln!("Cannot find file {}", filepath);
            return Ok(Self::new(Vec::new()));
        }

        let file = fs::File::open(filepath)?;
        let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(file);

        let mut entries = Vec::new();
        for log in reader.deserialize() {
            match log {
                Ok(log) => entries.push(log),
                Err(e) => {
                    eprintln!("Warning: Skipping malformed CSV record: {}", e);
                    continue;
                }
            }
        }

        Ok(Self::new(entries))
    }

    /// Writes the current entries to the given filepath.
    pub fn persist(&self, filepath: &String) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create(filepath).unwrap();
        let mut writer = csv::WriterBuilder::new().flexible(true).from_writer(file);
        for entry in self.entries.iter() {
            writer.serialize(entry)?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Updates the current entries and returns the result as new TimeLog
    pub fn update(&self) -> Self {
        let action = self.determine_action();
        self.apply_action(action)
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
    fn determine_action(&self) -> UpdateAction {
        match self.entries.last() {
            None => UpdateAction::NewDay(self.today, self.current_time),
            Some(TimeLogEntry { date: d, .. }) if d != &self.today => {
                UpdateAction::NewDay(self.today, self.current_time)
            }
            Some(last_entry) => {
                if last_entry.end_am.is_none() {
                    UpdateAction::FillSlot(TimeOfDay::EndAM, self.current_time)
                } else if last_entry.start_pm.is_none() {
                    UpdateAction::FillSlot(TimeOfDay::StartPM, self.current_time)
                } else if last_entry.end_pm.is_none() {
                    UpdateAction::FillSlot(TimeOfDay::EndPM, self.current_time)
                } else {
                    UpdateAction::NoChange
                }
            }
        }
    }

    /// Applies the given action and returns the result as a new TimeLog
    fn apply_action(&self, action: UpdateAction) -> Self {
        let new_entries = match action {
            UpdateAction::NoChange => self.entries.clone(),
            UpdateAction::NewDay(date, time) => self
                .entries
                .iter()
                .cloned()
                .chain(std::iter::once(TimeLogEntry::new(date, time)))
                .collect(),
            UpdateAction::FillSlot(time_of_day, time) => {
                let mut new_vec = self.entries.clone();
                let new_entry = new_vec.last_mut().unwrap();
                match time_of_day {
                    TimeOfDay::EndAM => {
                        new_entry.set_end_am(time);
                    }
                    TimeOfDay::StartPM => {
                        new_entry.set_start_pm(time);
                    }
                    TimeOfDay::EndPM => {
                        new_entry.set_end_pm(time);
                    }
                }
                new_vec
            }
        };

        Self {
            entries: new_entries,
            today: self.today,
            current_time: self.current_time,
        }
    }
}

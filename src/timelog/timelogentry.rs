//! Separates the TimeLogEntry struct from the main TimeLog module
use std::fmt::Display;

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

/// Represents the current state of a TimeLogEntry
#[derive(Debug, Clone, Default)]
pub enum DayState {
    #[default]
    FreshDay,
    MorningStarted(NaiveTime),
    MorningFinished(NaiveTime, NaiveTime),
    AfternoonStarted(NaiveTime, NaiveTime, NaiveTime),
    DayFinished(NaiveTime, NaiveTime, NaiveTime, NaiveTime),
}

/// Holds information about a single line of the CSV file
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(into = "TimeLogEntryDTO", from = "TimeLogEntryDTO")]
pub struct TimeLogEntry {
    pub(super) date: NaiveDate,
    pub(super) state: DayState,
}

impl TimeLogEntry {
    /// Creates a new TimeLogEntry, representing a brand new line in the file
    pub fn new(date: NaiveDate, start_am: NaiveTime) -> Self {
        Self {
            date,
            state: DayState::MorningStarted(start_am),
        }
    }

    pub fn transition(&mut self, time: NaiveTime) {
        let new_state = match self.state {
            DayState::FreshDay => DayState::MorningStarted(time),
            DayState::MorningStarted(start_am) => DayState::MorningFinished(start_am, time),
            DayState::MorningFinished(start_am, end_am) => {
                DayState::AfternoonStarted(start_am, end_am, time)
            }
            DayState::AfternoonStarted(start_am, end_am, start_pm) => {
                DayState::DayFinished(start_am, end_am, start_pm, time)
            }
            DayState::DayFinished(start_am, end_am, start_pm, end_pm) => {
                DayState::DayFinished(start_am, end_am, start_pm, end_pm)
            }
        };
        self.state = new_state;
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct TimeLogEntryDTO {
    date: NaiveDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    start_am: Option<NaiveTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    end_am: Option<NaiveTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    start_pm: Option<NaiveTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    end_pm: Option<NaiveTime>,
}

impl From<TimeLogEntryDTO> for TimeLogEntry {
    fn from(dto: TimeLogEntryDTO) -> Self {
        let state = match (dto.start_am, dto.end_am, dto.start_pm, dto.end_pm) {
            (Some(start_am), Some(end_am), Some(start_pm), Some(end_pm)) => {
                DayState::DayFinished(start_am, end_am, start_pm, end_pm)
            }
            (Some(start_am), Some(end_am), Some(start_pm), None) => {
                DayState::AfternoonStarted(start_am, end_am, start_pm)
            }
            (Some(start_am), Some(end_am), None, _) => DayState::MorningFinished(start_am, end_am),
            (Some(start_am), None, _, _) => DayState::MorningStarted(start_am),
            (None, _, _, _) => DayState::FreshDay,
        };
        TimeLogEntry {
            date: dto.date,
            state,
        }
    }
}

impl From<TimeLogEntry> for TimeLogEntryDTO {
    fn from(value: TimeLogEntry) -> Self {
        let mut dto = TimeLogEntryDTO { date: value.date, ..Default::default() };
        match value.state {
            DayState::MorningStarted(start_am) => dto.start_am = Some(start_am),
            DayState::MorningFinished(sa, ea) => { dto.start_am = Some(sa); dto.end_am = Some(ea); }
            DayState::AfternoonStarted(sa, ea, sp) => { dto.start_am = Some(sa); dto.end_am = Some(ea); dto.start_pm = Some(sp); }
            DayState::DayFinished(sa, ea, sp, ep) => { dto.start_am = Some(sa); dto.end_am = Some(ea); dto.start_pm = Some(sp); dto.end_pm = Some(ep); }
            DayState::FreshDay => (),
        }
        dto
    }
}

impl From<&TimeLogEntry> for TimeLogEntryDTO {
    fn from(value: &TimeLogEntry) -> Self {
        value.clone().into()
    }
}

impl Display for TimeLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dto: TimeLogEntryDTO = self.into();
        write!(f, "{}", dto)
    }
}

impl Display for TimeLogEntryDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:^14}{:^12}{:^12}{:^12}{:^12}", self.date.to_string(), self.start_am.map_or(String::new(), |t| t.to_string()), self.end_am.map_or(String::new(), |t| t.to_string()), self.start_pm.map_or(String::new(), |t| t.to_string()), self.end_pm.map_or(String::new(), |t| t.to_string()))
    }
}

use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime};

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
    pub fn new(date: NaiveDate, start_am: NaiveTime) -> Self {
        Self {
            date,
            start_am: Some(start_am),
            end_am: None,
            start_pm: None,
            end_pm: None,
        }
    }

    pub fn set_end_am(&mut self, end_am: NaiveTime) {
        self.end_am = Some(end_am);
    }

    pub fn set_start_pm(&mut self, start_pm: NaiveTime) {
        self.start_pm = Some(start_pm);
    }

    pub fn set_end_pm(&mut self, end_pm: NaiveTime) {
        self.end_pm = Some(end_pm);
    }
}

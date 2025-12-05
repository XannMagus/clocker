use chrono::{Local, NaiveDate, NaiveTime, Timelike};
use csv::{ReaderBuilder};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, io};

const DEFAULT_PATH: &str = "~/horaires.csv";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
struct TimeLogEntry {
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

impl TimeLogEntry {
    fn new(date: NaiveDate, start_am: NaiveTime) -> Self {
        Self {
            date,
            start_am: Some(start_am),
            end_am: None,
            start_pm: None,
            end_pm: None,
        }
    }

    fn set_end_am(&mut self, end_am: NaiveTime) {
        self.end_am = Some(end_am);
    }

    fn set_start_pm(&mut self, start_pm: NaiveTime) {
        self.start_pm = Some(start_pm);
    }

    fn set_end_pm(&mut self, end_pm: NaiveTime) {
        self.end_pm = Some(end_pm);
    }
}

#[derive(Debug)]
enum TimeOfDay {
    EndAM,
    StartPM,
    EndPM,
}

#[derive(Debug)]
enum UpdateAction {
    NewDay(NaiveDate, NaiveTime),
    FillSlot(TimeOfDay, NaiveTime),
    NoChange,
}

fn determine_action(entries: &Vec<TimeLogEntry>, date: NaiveDate, time: NaiveTime) -> UpdateAction {
    match entries.last() {
        None => UpdateAction::NewDay(date, time),
        Some(TimeLogEntry { date: d, .. }) if d != &date => UpdateAction::NewDay(date, time),
        Some(last_entry) => {
            if last_entry.end_am.is_none() {
                UpdateAction::FillSlot(TimeOfDay::EndAM, time)
            } else if last_entry.start_pm.is_none() {
                UpdateAction::FillSlot(TimeOfDay::StartPM, time)
            } else if last_entry.end_pm.is_none() {
                UpdateAction::FillSlot(TimeOfDay::EndPM, time)
            } else {
                UpdateAction::NoChange
            }
        }
    }
}

fn read_logs(filepath: &String) -> io::Result<Vec<TimeLogEntry>> {
    if !fs::metadata(&filepath).is_ok() {
        eprintln!("Cannot find file {}", filepath);
        return Ok(Vec::new());
    }

    let file = fs::File::open(filepath)?;
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);

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

    Ok(entries)
}

fn write_logs(filepath: &String, entries: Vec<TimeLogEntry>) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create(filepath).unwrap();
    let mut writer = csv::WriterBuilder::new().flexible(true).from_writer(file);
    for entry in entries {
        writer.serialize(entry)?;
    }
    writer.flush()?;
    Ok(())
}

fn apply_action(entries: Vec<TimeLogEntry>, action: UpdateAction) -> Vec<TimeLogEntry> {
    match action {
        UpdateAction::NoChange => entries,
        UpdateAction::NewDay(date, time) => entries
            .iter()
            .cloned()
            .chain(std::iter::once(TimeLogEntry::new(date, time)))
            .collect(),
        UpdateAction::FillSlot(time_of_day, time) => {
            let mut new_vec = entries.clone();
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
    }
}

fn main() {
    let filename = shellexpand::tilde(
        &env::args()
            .skip(1)
            .next()
            .unwrap_or(DEFAULT_PATH.to_string()),
    )
    .to_string();
    let now = Local::now();
    let today = now.date_naive();
    let current_time = now
        .time()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();

    let logs = read_logs(&filename).unwrap();
    let action = determine_action(&logs, today, current_time);
    let new_logs = apply_action(logs, action);
    let _ = write_logs(&filename, new_logs).expect("Problem writing the file");
}

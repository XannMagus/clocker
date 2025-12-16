# Clocker

A simple CLI utility to track work time in CSV files.

Clocker works like a primitive clocking machine—run it once to clock in, run it again to clock out. It supports a morning session and an afternoon session, giving you four time slots per day: start AM, end AM, start PM, and end PM.

## Installation

```bash
git clone https://github.com/XannMagus/clocker.git
cd clocker
cargo install --path .
```

This builds an optimized release binary and installs it to `~/.cargo/bin/`.

## Usage

Simply run the program to log the current time:

```bash
clocker
```

This appends the current time to the next available slot in `~/timelog.csv`.

### Custom file paths

```bash
clocker <INPUT_FILE> [OUTPUT_FILE]
```

- `INPUT_FILE` — Path to read existing entries from (default: `~/timelog.csv`)
- `OUTPUT_FILE` — Path to write updated entries to (default: same as input)

### Examples

```bash
# Use default file
clocker

# Use a custom file
clocker ~/work/timelog.csv

# Read from one file, write to another
clocker ~/work/timelog.csv ~/work/timelog_backup.csv
```

## How it works

Each day has four time slots filled in order:

1. **Start AM** — Morning clock-in
2. **End AM** — Morning clock-out
3. **Start PM** — Afternoon clock-in
4. **End PM** — Afternoon clock-out

When you run `clocker`:

- If today has no entry yet, a new row is created with the current time as Start AM
- If the current day's row exists, the next empty slot is filled
- If all slots are filled, no changes are made

## CSV format

The output CSV has five columns:

```csv
date,start_am,end_am,start_pm,end_pm
2025-01-15,08:30,12:00,13:00,17:30
2025-01-16,09:00,12:15,,
```

Empty fields are left blank when not yet filled.

## License

MIT License — see [LICENSE](LICENSE) for details.

---

*This README was generated with the assistance of AI.*

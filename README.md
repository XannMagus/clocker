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

### Logging time

Simply run the program to log the current time:

```bash
clocker
# or explicitly
clocker log
```

This appends the current time to the next available slot in `~/timelog.csv`.

### Viewing entries

View your timelog entries:

```bash
# View all entries
clocker view
# or explicitly
clocker view all

# View only the latest entry
clocker view latest
```

### Archiving

Archive your current timelog and start fresh:

```bash
clocker archive
```

This will:
1. Update the current day's entry
2. Create a backup of the file with a `.bak` extension
3. Initialize a new empty timelog

### Starting a new month

Move the current timelog to archive without logging the current time:

```bash
clocker new-month
```

This will:
1. Create a backup of the current file
2. Initialize a new empty timelog with today's entry

### Custom file paths

All commands support custom input and output file paths:

```bash
clocker -i <INPUT_FILE> -o <OUTPUT_FILE> [COMMAND]
```

- `-i, --input-file` — Path to read existing entries from (default: `~/timelog.csv`)
- `-o, --output-file` — Path to write updated entries to (default: same as input)

### Examples

```bash
# Use default file
clocker

# Use a custom file
clocker -i ~/work/timelog.csv

# Read from one file, write to another
clocker -i ~/work/timelog.csv -o ~/work/timelog_new.csv

# View entries from a specific file
clocker -i ~/work/timelog.csv view all

# Archive with custom file
clocker -i ~/work/timelog.csv archive
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
- If all slots are filled, you'll receive an error: "Shift already complete for today."

## CSV format

The output CSV has five columns:

```csv
date,start_am,end_am,start_pm,end_pm
2025-01-15,08:30,12:00,13:00,17:30
2025-01-16,09:00,12:15,,
```

Empty fields are left blank when not yet filled.

## Error handling

Clocker provides clear error messages for common issues:

- **Shift already complete** — All four time slots for today are filled
- **Malformed CSV** — The input file contains invalid data
- **File errors** — Issues reading or writing files

## License

MIT License — see [LICENSE](LICENSE) for details.

---

*This README was generated with the assistance of AI.*

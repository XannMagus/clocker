## Description

Clocker is a simple CLI utility to track work time in CSV files.

This works like a very primitive clocking machine, with a clock-in/clock-out pair for the morning, and another for the afternoon.

## Usage

Simply run the program with no arguments to have it add the current time as the last entry in the document.

```bash
clocker
```


The default file path is `~/horaires.csv`.

The program takes two possible arguments:
1. the input file path
2. the output file path (defaults to input file path if omitted)



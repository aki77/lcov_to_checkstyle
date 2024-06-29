# lcov_to_checkstyle

`lcov_to_checkstyle` is a tool designed to convert LCOV coverage data into a Checkstyle-compatible XML format.
It is implemented in Rust and is intended for use in CI/CD pipelines.

## Features

- **Fast Conversion**: Leveraging Rust's performance to quickly process even large coverage data files.
- **Easy Integration**: The Checkstyle format is supported by many CI tools, making it easy to integrate into existing workflows.

## Usage


lcov_to_checkstyle is provided as a command-line tool. The basic usage is as follows:

```
lcov_to_checkstyle <LCOV_FILE>
```

- `<LCOV_FILE>`: The path to the LCOV coverage data file you want to convert.

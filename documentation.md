# Documentation

## 1. Overview

LogTidy converts unstructured log lines into structured JSON. It is designed for predictable CLI behaviour, simple composition with other tools, and clean integration into pipelines. This document provides full usage guidance, examples, and troubleshooting notes.


LogTidy provides two primary modes of operation:

- **Streaming mode** — reads from standard input line‑by‑line  
- **Batch mode** — processes an entire file at once  

Both modes emit structured JSON envelopes suitable for downstream tooling, ingestion pipelines, and log processors.

Two binaries are available:

- `logtidy` — full command name  
- `lt` — short alias for daily use  

Both map to the same executable logic.

---

## 2. Installation

Build the release binaries:

`cargo build --release`

Install them into a directory on your `PATH` (for example, Cargo’s bin directory):

`cp target/release/logtidy.exe ~/.cargo/bin/`
`cp target/release/lt.exe ~/.cargo/bin/`


Verify installation:

`logtidy`
`lt`


---

## 3. Commands

LogTidy exposes a minimal, intentional command surface:

| Command | Description |
|---------|-------------|
| `stream` | Read from stdin line‑by‑line and emit structured JSON |
| `batch`  | Process an entire file at once |
| `sample` | Display example input and output |
| `docs`   | Show documentation reference |
| `about`  | Display tool information |
| `help`   | Show help for any command |

---

## 4. Usage

### 4.1. Streaming Mode

Streaming mode reads from standard input and processes each line independently.

Example:

`Get-Content sample.log | logtidy stream`

Short form:

`Get-Content sample.log | lt stream`


Streaming mode is ideal for:

- piping logs from other tools  
- tailing files  
- real‑time processing  
- integration with shell pipelines  


### 4.2. Batch Mode

Batch mode processes a file in one operation.

`logtidy batch -f sample.log`

or:

`lt batch -f sample.log`


Batch mode is useful when:

- the entire file must be parsed as a unit  
- deterministic, file‑based processing is required  
- working with archived logs  


### 4.3. Sample Output

To view example input and output:

`logtidy sample`

This is useful for verifying installation or understanding the output structure.


### 4.4. Documentation and About

Show documentation reference:

`logtidy docs`

Show tool information:

`logtidy about`



---

## 5. Output Format

LogTidy emits structured JSON envelopes. Each processed line is wrapped in a consistent structure containing:

- the original line  
- parsed fields (when available)  
- normalised representations  
- metadata  

The envelope format is stable and predictable, even as parsing logic evolves.

---

## 6. Development Notes

LogTidy is implemented in Rust using:

- **Clap** for CLI parsing  
- **Serde** for JSON output  
- **Regex** for pattern matching  
- **ThisError / anyhow** for error handling  

Integration tests use:

- `assert_cmd`  
- `predicates`  

Fixtures live under `tests/fixtures/`.

---

## 7. Troubleshooting

### 7.1. LogTidy is not recognised as a command

Ensure Cargo’s bin directory is on your `PATH`:

`~/.cargo/bin`


Restart your terminal after installation.


### 7.2. Streaming mode produces no output

Confirm that input is being piped correctly:

`"INFO test" | logtidy stream`


If this works, the issue is with the upstream command.


### 7.3. Batch mode cannot find the file

Ensure the path is correct and quoted if it contains spaces:

`logtidy batch -f "C:\logs\sample log.txt"`


### 7.4. Windows PowerShell considerations

PowerShell may buffer output differently from Bash.  

Use `Get-Content` for consistent streaming behaviour:

`Get-Content sample.log | logtidy stream`


---

## 8. Versioning

Current version: `0.1.0` — In Development


# Documentation

## 1. Overview

LogTidy converts unstructured and JSON log lines into predictable, structured JSONL. It is designed for simple CLI use, clean composition with other tools, and reliable behaviour in pipelines. The scope is intentionally small: take whatever text you give it, normalise what can be normalised, and emit a stable JSON envelope for each non-empty line.

To keep LogTidy predictable, it intentionally does **not**:

- infer log levels  
- infer meaning from message text  
- classify errors or warnings  
- detect slow operations  
- reorder lines  
- drop malformed lines (except blank/whitespace-only lines; see Section 5.6)

LogTidy is a normaliser, not an analyser. It prepares logs for downstream tools without making assumptions about their meaning.

Two processing modes are available:

- **Streaming mode**: process input line by line from stdin  
- **Batch mode**: process an entire file (or stdin) at once  

---

## 2. Prerequisites

Before installing LogTidy, ensure the following requirements are met:

### 2.1 A terminal
LogTidy is a command‑line tool. You will need:

- **Windows**: PowerShell or Command Prompt (including VS Code's integrated terminal)  
- **macOS**: Terminal  
- **Linux**: Any shell (bash, zsh, etc.)

### 2.2 Understanding PATH
Your **PATH** is the list of folders your system searches when you type a command.

If a program is inside a folder on your PATH, you can run it from any directory:

```
logtidy help
```

If it is *not* on your PATH, you must run it using the full file path:

```
C:\Users\<you>\Downloads\logtidy.exe help
```

### 2.3 A folder on PATH
You will need a folder where you can place the LogTidy binaries (`logtidy` and `lt`).

If you have Rust installed, the Cargo bin directory is already on PATH:

- **Windows**: `C:\Users\<you>\.cargo\bin\`
- **Linux/macOS**: `~/.cargo/bin/`

Placing the binaries here makes them available globally.

### 2.4 Rust and Cargo (optional on Windows; required for source builds)
Rust is **not required** if you download a prebuilt binary from GitHub Releases, but it is required if you want to:

- build or install from source (`cargo build --release` or `cargo install --path .`)
- use the `.cargo/bin` directory (already on PATH)

---

## 3. Installation

LogTidy can currently be installed by downloading a prebuilt binary from **GitHub Releases** or building from source with **Cargo**. Publishing to `crates.io` (`cargo install logtidy`) is planned for a future release.

---

### 3.1 Install from GitHub Releases

Download the latest release from:

https://github.com/musman-uk/logtidy/releases

**Note:**  
Currently only **Windows binaries** (`logtidy.exe` and `lt.exe`) are provided on GitHub Releases.  
Linux and macOS users should build from source using Cargo (Section 3.2).

---

### 3.1.1 Windows

1. Download `logtidy.exe` (and optionally the `lt.exe` shorthand binary) from the release.
2. Move the binary into your Cargo bin directory (or another folder on your PATH):

```
C:\Users\<you>\.cargo\bin\
```

3. Verify installation:

```
logtidy help
```

If you prefer not to move it, you can run it directly from the download location:

```
.\logtidy.exe help
```

#### Adding a custom folder to PATH (optional)

If you want to keep the binary somewhere else:

1. Open **Start Menu → Environment Variables**  
2. Click **Environment Variables…**  
3. Under **User variables**, select **Path** → **Edit**  
4. Click **New** and add the folder containing `logtidy.exe`  
5. Restart your terminal  

**Note:**  
As LogTidy is a command‑line tool, double-clicking the `.exe` file will briefly open and close a window immediately — run it from a terminal instead.

---

### 3.2 Build from source with Cargo

Clone the repository and build release binaries:

```
git clone https://github.com/musman-uk/logtidy.git
cd logtidy
cargo build --release
```

Copy both binaries (`logtidy` and `lt`) to a folder on your PATH:

**Linux / macOS:**
```
cp target/release/logtidy target/release/lt ~/.cargo/bin/
```

**Windows (PowerShell):**
```
Copy-Item .\target\release\logtidy.exe, .\target\release\lt.exe $env:USERPROFILE\.cargo\bin\
```

Alternatively, install directly from the cloned directory with Cargo:

```
cargo install --path .
```

Verify installation:

```
logtidy help
```

---

### 3.3 Installing via crates.io (Planned)

Support for installing directly from `crates.io` (`cargo install logtidy`) is planned and not yet live. Until published, use GitHub Releases or build from source.

---

### 3.4 Quick Start

Stream a log file:

```
cat app.log | logtidy stream
```

PowerShell:

```
Get-Content app.log | logtidy stream
```

This emits structured JSONL for each non-empty line.

---

## 4. Commands

| Command  | Description                                                      |
|----------|------------------------------------------------------------------|
| `stream` | Read from stdin and emit JSON per line                           |
| `batch`  | Process a file (`-f` / `--file`) or stdin at once                |
| `sample` | Show example input and output                                    |
| `docs`   | Display condensed documentation reference                        |
| `about`  | Show tool name, version, license, source, and copyright notice   |
| `help`   | Show help for any command                                        |

The command surface is intentionally minimal.

---

## 5. Usage

### 5.1 Streaming Mode

Process logs line by line from stdin:

```
Get-Content sample.log | logtidy stream
```

Short form:

```
Get-Content sample.log | lt stream
```

Use streaming mode when:

- piping logs from other tools  
- tailing files  
- processing logs in real time  
- composing shell pipelines  

---

### 5.2 Batch Mode

Process an entire file:

```
logtidy batch -f sample.log
```

Long flag form:

```
logtidy batch --file sample.log
```

If `-f` / `--file` is omitted, `batch` reads from `stdin` until EOF.

Useful for:

- archived logs  
- deterministic file-based processing  
- tooling that expects whole-file input  

---

### 5.3 Example Output

**Plain-text line with timestamp and level:**

Input:

```
2024-03-10T12:00:01Z INFO Service started
```

Output:

```
{"line":1,"message":"Service started","timestamp":"2024-03-10T12:00:01Z","level":"INFO","fields":null}
```

**Unstructured plain-text line:**

Input:

```
Hello world
```

Output:

```
{"line":1,"message":"Hello world","timestamp":null,"level":null,"fields":null}
```

**Structured JSON line:**

Input:

```
{"ts":"2024-03-10T12:00:04Z","level":"info","message":"Cache warm","items":120}
```

Output:

```
{"line":1,"message":"Cache warm","timestamp":"2024-03-10T12:00:04Z","level":"INFO","fields":{"items":120}}
```

---

### 5.4 Documentation and About

Show condensed documentation:

```
logtidy docs
```

Show tool information, version, and license:

```
logtidy about
```
---

## 5.5 Output Format

Each processed line is emitted as a JSON object with keys in a fixed order:

- **line**: 1-based input line number  
- **message**: remaining message text after extracting timestamp and level (or JSON `"message"` / `"msg"`), or `null` if empty  
- **timestamp**: extracted timestamp or `null`  
- **level**: extracted uppercase log level or `null`  
- **fields**: JSON object containing any additional keys from JSON input lines, or `null` for plain-text lines  

---

## 5.6 Behaviour on Malformed or Irregular Input

LogTidy aims to be predictable:

- **Blank lines are skipped**: lines that are empty or contain only whitespace are omitted from output, though they still increment the 1-based `line` counter  
- **Plain-text timestamp format**: only ISO-8601 UTC timestamps matching `YYYY-MM-DDTHH:MM:SSZ` are extracted; non-matching timestamps remain part of `message` and `timestamp` is `null`  
- **Plain-text log levels**: only uppercase `INFO`, `WARN`, `ERROR`, `DEBUG`, and `TRACE` are extracted; lowercase or mixed-case words in plain text are left untouched  
- **Message extraction on plain text**: when a timestamp or level is matched in plain text, it is stripped from `message` and surrounding whitespace is trimmed (if nothing remains, `message` becomes `null`)  
- **JSON line handling**: lines beginning with `{` (ignoring leading whitespace) are parsed as JSON objects. Keys `ts`/`timestamp`/`time`, `level`/`lvl` (normalised to uppercase), and `message`/`msg` map to the top-level envelope, and any remaining keys are placed in `fields`  
- **Graceful fallback on broken JSON**: if a line starts with `{` but is not valid JSON, LogTidy does not fail—it falls back to plain-text extraction  
- **Whitespace differences between modes**: `batch` trims leading and trailing whitespace on each line before parsing, whereas `stream` preserves leading spaces in plain-text messages  
- **Input must be valid UTF‑8**: invalid UTF‑8 byte sequences return an I/O error rather than panicking  

---

## 6. Platform Notes

### Windows PowerShell

Use `Get-Content` for consistent streaming:

```
Get-Content sample.log | logtidy stream
```

### Linux and macOS

Standard piping works as expected:

```
cat sample.log | logtidy stream
```

---

## 7. Troubleshooting

### LogTidy is not recognised as a command

Ensure your Cargo bin directory is on your PATH:

- **Windows**: `C:\Users\<you>\.cargo\bin\`
- **Linux/macOS**: `~/.cargo/bin/`

Restart your terminal after updating PATH.

### Streaming mode produces no output

Test with a simple inline example:

```
"2024-03-10T12:00:01Z INFO test" | logtidy stream
```

If this outputs a JSON line, the issue is upstream.

### Batch mode cannot find the file

Quote paths that contain spaces:

```
logtidy batch -f "C:\logs\sample log.txt"
```

### PowerShell buffering issues

Use `Get-Content` rather than `type` or `cat` equivalents.

---

## 8. Known Limitations

- Plain-text extraction only supports one timestamp format (`YYYY-MM-DDTHH:MM:SSZ`) and uppercase log levels (`INFO`, `WARN`, `ERROR`, `DEBUG`, `TRACE`)  
- Blank lines are skipped rather than emitted  
- Input must be valid UTF‑8  
- JSON detection only inspects lines starting with `{`  
- LogTidy does not infer meaning, severity, or structure beyond what is explicitly present  

---

## 9. Summary

LogTidy is a predictable tool. It takes messy log lines and turns them into clean, structured JSONL without guessing or interpreting meaning. It is intentionally minimal, pipeline-friendly, and designed to pair naturally with downstream tools, making it safe to integrate into real workflows.

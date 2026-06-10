# Documentation

## 1. Overview

LogTidy converts unstructured log lines into predictable, structured JSON. It is designed for simple CLI use, clean composition with other tools, and reliable behaviour in pipelines. The scope is intentionally small: take whatever text you give it, normalise what can be normalised, and emit a stable JSON envelope for each line.

To keep LogTidy predictable, it intentionally does **not**:

- infer log levels  
- infer meaning from message text  
- classify errors or warnings  
- detect slow operations  
- modify message content  
- reorder lines  
- drop malformed lines (except empty lines, see notes below)

LogTidy is a normaliser, not an analyser. It prepares logs for downstream tools without making assumptions about their meaning.

Two modes are available:

- **Streaming mode**: process input line by line from stdin  
- **Batch mode**: process an entire file at once  

---

## 2. Prerequisites

Before installing LogTidy, there are a few requirements to ensure installation can run smoothly:

### 2.1 A terminal
LogTidy is a command‑line tool. You will need:

- **Windows**: PowerShell or Command Prompt  
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
You will need a folder where you can place the LogTidy binary.

If you have Rust installed, the Cargo bin directory is already on PATH:

- **Windows**: `C:\Users\<you>\.cargo\bin\`
- **Linux/macOS**: `~/.cargo/bin/`

Placing the binary here makes it available globally.

### 2.4 Rust and Cargo (optional)
Rust is **not required** to run LogTidy, but it is useful if you want to:

- install via `cargo install logtidy`
- build from source
- use the `.cargo/bin` directory (already on PATH)

---

## 3. Installation

LogTidy can be installed in three ways: downloading a binary from GitHub Releases, installing from crates.io, or building from source using Cargo.

All installation methods assume the prerequisites above have been met.

---

### 3.1 Install from GitHub Releases

Download the latest binary from the Releases page:

https://github.com/musman-uk/logtidy/releases

**Note:**  
Currently only **Windows binaries** are provided.  
Linux and macOS users must install via crates.io or build from source.

---

### 3.1.1 Windows

1. Download `logtidy.exe` from the release.
2. Move it into your Cargo bin directory:

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

---

### 3.2 Install from crates.io

If you have Rust and Cargo installed:

```
cargo install logtidy
```

This compiles LogTidy from source and places the binary in `~/.cargo/bin`.

---

### 3.3 Build from source with Cargo

Clone the repository and build a release binary:

```
cargo build --release
```

Then move the binary to a folder on your PATH:

```
cp target/release/logtidy ~/.cargo/bin/
```

Verify installation:

```
logtidy help
```

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

This emits structured JSON for each line.

---

## 4. Commands

| Command   | Description                               |
|----------|-------------------------------------------|
| `stream` | Read from stdin and emit JSON per line    |
| `batch`  | Process a file in one operation           |
| `sample` | Show example input and output             |
| `docs`   | Display documentation reference           |
| `about`  | Show tool information                     |
| `help`   | Show help for any command                 |

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

Useful for:

- archived logs  
- deterministic file based processing  
- tooling that expects whole file input  

---

### 5.3 Example Output

Input:

```
12:00:01 INFO Service started
```

Output:

```
{"line":1,"message":"12:00:01 INFO Service started","timestamp":"12:00:01","level":"INFO","fields":null}
```

Unstructured lines are still emitted:

Input:

```
Hello world
```

Output:

```
{"line":1,"message":"Hello world","timestamp":null,"level":null,"fields":null}
```

---

## 5.4 Documentation and About

Show documentation:

```
logtidy docs
```

Show tool information:

```
logtidy about
```

---

## 5.5 Output Format

Each processed line is emitted as a JSON object with:

- **line**: line number starting at 1  
- **message**: original text (minus trailing newline)  
- **timestamp**: extracted timestamp or null  
- **level**: extracted log level or null  
- **fields**: reserved for future structured fields  

The format is stable and predictable across versions.

---

## 5.6 Behaviour on Malformed or Irregular Input

LogTidy aims to be predictable:

- **Empty lines are skipped** due to Rust `.lines()` behaviour  
- **Invalid timestamps become null**  
- **Only one timestamp format is supported**:  
  `YYYY-MM-DDTHH:MMSSZ`  
- **Only uppercase log levels are detected**:  
  `INFO`, `WARN`, `ERROR`, `DEBUG`, `TRACE`  
- **Lowercase or mixed case levels are not detected**  
- **Input must be valid UTF‑8**; invalid byte sequences cause an error before processing  
- **JSON is only detected when the line begins with `{`**  
- **No parsing errors are thrown for malformed text**, only for invalid UTF‑8  

This keeps LogTidy deterministic and safe for pipelines.

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

Ensure Cargo bin directory is on your PATH:

```
~/.cargo/bin
```

Restart your terminal after installation.

### Streaming mode produces no output

Test with a simple inline example:

```
"INFO test" | logtidy stream
```

If this works, the issue is upstream.

### Batch mode cannot find the file

Quote paths that contain spaces:

```
logtidy batch -f "C:\logs\sample log.txt"
```

### PowerShell buffering issues

Use `Get-Content` rather than `type` or `cat` equivalents.

---

## 8. Known Limitations

- Only one timestamp format is supported:  
  `YYYY-MM-DDTHH:MMSSZ`
- Only uppercase log levels are detected  
- Empty lines are skipped rather than emitted  
- Input must be valid UTF‑8  
- JSON is only detected when the line begins with `{`  
- LogTidy does not infer meaning, severity, or structure beyond what is explicitly present  

---

## 9. Summary

LogTidy is a predictable tool. It takes messy log lines and turns them into clean, structured JSON without guessing or interpreting meaning. It is intentionally minimal, pipeline friendly and designed to pair naturally with downstream tools, making it safe to integrate into real workflows.

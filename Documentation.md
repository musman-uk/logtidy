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

Two binaries are provided:

- `logtidy`: full command  
- `lt`: short alias  

Both behave the same.

---

## 2. Installation

Build release binaries:

```
cargo build --release
```

Install them somewhere on your PATH:

```
cp target/release/logtidy ~/.cargo/bin/
cp target/release/lt ~/.cargo/bin/
```

Verify installation:

```
logtidy help
lt help
```

### Quick Start

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

## 3. Commands

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

## 4. Usage

### 4.1 Streaming Mode

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

### 4.2 Batch Mode

Process an entire file:

```
logtidy batch -f sample.log
```

Useful for:

- archived logs  
- deterministic file based processing  
- tooling that expects whole file input  

---

### 4.3 Example Output

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

## 4.4 Documentation and About

Show documentation:

```
logtidy docs
```

Show tool information:

```
logtidy about
```

---

## 4.5 Output Format

Each processed line is emitted as a JSON object with:

- **line**: line number starting at 1  
- **message**: original text (minus trailing newline)  
- **timestamp**: extracted timestamp or null  
- **level**: extracted log level or null  
- **fields**: reserved for future structured fields  

The format is stable and predictable across versions.

---

## 4.6 Behaviour on Malformed or Irregular Input

LogTidy aims to be predictable:

- **Empty lines are skipped** due to Rust `.lines()` behaviour  
- **Invalid timestamps become null**  
- **Only one timestamp format is supported**:  
  `YYYY-MM-DDTHH:MM:SSZ`  
- **Only uppercase log levels are detected**:  
  `INFO`, `WARN`, `ERROR`, `DEBUG`, `TRACE`  
- **Lowercase or mixed case levels are not detected**  
- **Input must be valid UTF 8**; invalid byte sequences cause an error before processing  
- **JSON is only detected when the line begins with `{`**  
- **No parsing errors are thrown for malformed text**, only for invalid UTF 8  

This keeps LogTidy deterministic and safe for pipelines.

---

## 5. Platform Notes

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

## 6. Troubleshooting

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

## 7. Known Limitations

- Only one timestamp format is supported:  
  `YYYY-MM-DDTHH:MM:SSZ`
- Only uppercase log levels are detected  
- Empty lines are skipped rather than emitted  
- Input must be valid UTF 8  
- JSON is only detected when the line begins with `{`  
- LogTidy does not infer meaning, severity, or structure beyond what is explicitly present  

---

## 8. Summary

LogTidy is a predictable tool. It takes messy log lines and turns them into clean, structured JSON without guessing or interpreting meaning. It is intentionally minimal, pipeline friendly and designed to pair naturally with downstream tools, making it safe to integrate into real workflows.

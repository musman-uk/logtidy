# LogTidy Documentation

## 1. Overview

LogTidy converts unstructured log lines into predictable, structured JSON. It is designed for simple CLI use, clean composition with other tools, and reliable behaviour in pipelines. The tool keeps its scope intentionally small: take whatever text you give it, normalise what can be normalised, and emit a stable JSON envelope for each line.

To keep LogTidy predictable, it intentionally does **not**:

- infer log levels  
- infer meaning from message text  
- classify errors or warnings  
- detect slow operations  
- modify message content  
- reorder lines  
- drop malformed lines  

LogTidy is a normaliser, not an analyser. It prepares logs for downstream tools without making assumptions about their meaning.

Two modes are available:

- Streaming mode, process input line by line from stdin  
- Batch mode, process an entire file at once  

Two binaries are provided:

- `logtidy`, full command  
- `lt`, short alias  

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

Check that everything is wired up:

```
logtidy help
lt help
```

### Quick Start

If you want to see LogTidy in action immediately:

```
cat app.log | logtidy stream
```

Or on PowerShell:

```
Get-Content app.log | logtidy stream
```

This streams your log file and outputs structured JSON line by line.

---

## 3. Commands

| Command   | Description |
|-----------|-------------|
| `stream`  | Read from stdin and emit structured JSON per line |
| `batch`   | Process a file in one operation |
| `sample`  | Show example input and output |
| `docs`    | Display documentation reference |
| `about`   | Show tool information |
| `help`    | Show help for any command |

The command surface is intentionally minimal.

---

## 4. Usage

### 4.1 Streaming Mode

Streaming mode processes logs line by line from stdin:

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

Batch mode processes an entire file:

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

Malformed or unstructured lines are still emitted:

Input:

```
Hello world 
```

Output:

```
{"line":1,"message":"Hello world","timestamp":null,"level":null,"fields":null}
```

LogTidy is designed to always show the line numbers.

---

### 4.4 Documentation and About

Show documentation:

```
logtidy docs
```

Show tool information:

```
logtidy about
```

---

### 4.5 Output Format

Each processed line is emitted as a JSON object with:

- line, line number starting at 1  
- message, original text  
- timestamp, extracted timestamp or null  
- level, extracted log level or null  
- fields, reserved for future structured fields  

The format is stable and predictable across versions.

---

### 4.6 Behaviour on Malformed Input

As LogTidy is designed to be predictable:

- Lines are never dropped  
- Invalid timestamps become null  
- Missing levels become null  
- Empty lines are emitted as empty messages  
- No parsing errors are thrown during streaming  

This makes LogTidy safe for real world logs, where structure can often be inconsistent.

---

## 5. Platform Notes

### Windows PowerShell

Use Get-Content for consistent streaming:

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

Ensure Cargo’s bin directory is on your PATH:

```
~/.cargo/bin
```

Restart your terminal after installation.

### Streaming mode produces no output

Test with a simple inline example:

```
"INFO test" | logtidy stream
```

If this works, the issue is with the upstream command.

### Batch mode cannot find the file

Quote paths that contain spaces:

```
logtidy batch -f "C:\logs\sample log.txt"
```

### PowerShell buffering issues

Use Get-Content rather than type or cat equivalents.

---

## 7. Summary

LogTidy is a small, focused tool. It takes messy log lines and turns them into clean, structured JSON without guessing or interpreting meaning. It is predictable, pipeline friendly, and designed to pair naturally with downstream tools like FrictionDetect. It aims to be both easy to use and efficient within real workflows and pipelines.

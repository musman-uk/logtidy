# LogTidy

LogTidy is a log normaliser that  converts unstructured log lines into structured JSONL. It is designed for predictable CLI behaviour, simple composition with other tools, and clean integration into pipelines.

---

## Project Contents

A short summary of what each part of the project contains:

- `src/` – Rust source files for the CLI, parsing logic, and output envelope  
- `tests/` – integration tests and sample fixtures  
- `documentation.md` – full usage, examples, and troubleshooting  
- `Cargo.toml` – package metadata and binary definitions (`logtidy` and `lt`)  
- `LICENSE` – project license  
- `README.md` – high‑level overview of the project, including purpose and structure  

---

## Development Overview

A simplified view of how the tool is developed:

| Stage     | Location          | Purpose |
|-----------|-------------------|---------|
| Implement | `src/`            | Core logic, CLI behaviour, and parsing modules |
| Test      | `tests/`          | Integration tests and fixture‑based validation |
| Build     | `cargo build`     | Produces binaries (`logtidy`, `lt`) for release |
| Document  | `documentation.md`| Full usage, examples, and troubleshooting |

---

## Status

Version `0.1.0` – In Development


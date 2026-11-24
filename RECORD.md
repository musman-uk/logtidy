# RECORD

## 1. Design Principles

### Tool Identity
- Command‑line utility for transforming raw logs into structured, predictable output  
- Clear, minimal command set with consistent behaviour across modes  
- Output designed for readability and downstream processing  
- No implicit transformations; all normalisation steps are explicit and intentional  

### Interaction Model
- Commands follow a predictable pattern: heading → separator → content  
- Batch and stream modes reflect two primary usage contexts  
- Documentation and examples remain concise and directly actionable  
- Defaults avoid side effects and require no configuration to begin using the tool  

### Output Philosophy
- JSONL used as the primary structured format  
- Deterministic output across runs and environments  
- Errors surfaced with context to aid debugging  
- Examples demonstrate expected transformations without unnecessary detail  

---

## 2. Development Process

### Architecture
- Modular Rust codebase with clear separation of concerns  
- CLI layer implemented via `clap` and kept intentionally thin  
- Core logic encapsulated in modules (`parser`, `line`, `envelope`, `batch`, `stream`)  
- Error handling unified through `anyhow` for consistent failure modes  

### Command Structure
- `batch` – processes an entire file at once  
- `stream` – processes stdin line‑by‑line  
- `sample` – provides minimal example input and output  
- `docs` – summarises usage, inputs, outputs, and examples  
- `about` – displays tool name, version, license, repository source, and copyright

### Source Layout
| File / Module     | Purpose |
|-------------------|---------|
| `src/main.rs`     | CLI entry point; command dispatch for all modes. |
| `src/stream.rs`   | Streaming mode: processes stdin line‑by‑line and emits structured JSONL. |
| `src/batch.rs`    | Batch mode: processes an entire file at once. |
| `src/parser.rs`   | Core parsing logic for transforming raw log lines into structured representations. |
| `src/line.rs`     | Line model and helpers for normalisation and field extraction. |
| `src/envelope.rs` | Output envelope: wraps parsed lines into consistent JSON structures. |
| `src/errors.rs`   | Error types and error handling used across the tool. |

### Versioning
- Version sourced from `Cargo.toml` via `env!("CARGO_PKG_VERSION")`  
- Current stage: **v0.1.1**  
- Versioning will stabilise once parsing rules and output formats mature

---

## 3. Milestones

### Initial Structure
- Repository initialised with modular Rust layout  
- Core parsing and normalisation logic drafted  
- Batch and stream modes implemented to validate architecture  
- Early examples created to test output consistency  

### CLI & Behaviour
- Command definitions stabilised  
- Help, about, and docs commands implemented  
- License command added for authorship and rights declaration  
- Default command output aligned with overall tool behaviour  

### Documentation & Workflow
- README drafted to outline purpose and usage  
- RECORD.md introduced to track design and development decisions  
- Pre‑release version tagged as v0.1.0  
- Repository prepared for iterative refinement and future expansion  

---

## 4. Version History

### v0.1.1 – Current
- Relicensed under GNU General Public License v3.0 (`GPL-3.0-only`)
- Consolidated tool metadata, license, and copyright notice into `about`
- Aligned `README.md` and `Documentation.md` with current CLI behaviour and release workflow
- Set `default-run = "logtidy"` in `Cargo.toml`


### v0.1.0
- Implemented batch and stream processing modes  
- Added example and documentation commands  
- Introduced identity commands (`about`, `license`)  
- Established consistent CLI output structure  
- Finalised initial parsing and normalisation pipeline  
- Prepared repository for future refinement and public release  

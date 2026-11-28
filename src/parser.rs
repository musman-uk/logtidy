use crate::envelope::Envelope;
use crate::line::Line;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

/// Regexes are compiled once on construction so per-line processing avoids recompilation.
pub struct Parser {
    ts_regex: Regex,
    level_regex: Regex,
}

impl Parser {
    pub fn new() -> Self {
        // Intentionally scoped to strict ISO-8601 UTC timestamps and uppercase levels
        // to avoid false positives on arbitrary numbers or prose words inside messages.
        let ts_regex = Regex::new(
            r"(?P<ts>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z)"
        )
        .unwrap();

        let level_regex = Regex::new(r"\b(INFO|WARN|ERROR|DEBUG|TRACE)\b").unwrap();

        Self {
            ts_regex,
            level_regex,
        }
    }

    /// JSON keys take precedence when present; plain-text extractors backfill any missing fields.
    pub fn process(&self, line: Line) -> Value {
        let mut env = Envelope::new(line.number);

        if let Some(json_value) = self.try_json(&line.text) {
            self.extract_from_json(&mut env, json_value);
        }

        if env.timestamp.is_none() {
            if let Some(ts) = self.extract_timestamp(&line.text) {
                env.timestamp = Some(ts);
            }
        }

        if env.level.is_none() {
            if let Some(level) = self.extract_level(&line.text) {
                env.level = Some(level);
            }
        }

        if env.message.is_none() {
            if let Some(msg) = self.extract_message(&line.text) {
                env.message = Some(msg);
            }
        }

        env.into_json()
    }

    /// Guarding on a leading `{` avoids unnecessary JSON parsing on plain-text lines
    /// and prevents JSON scalars or arrays from bypassing plain-text extraction.
    /// Invalid JSON returns `None` so broken lines degrade to plain-text parsing rather than failing.
    fn try_json(&self, text: &str) -> Option<Value> {
        if !text.trim_start().starts_with('{') {
            return None;
        }
        serde_json::from_str::<Value>(text).ok()
    }

    fn extract_from_json(&self, env: &mut Envelope, value: Value) {
        if let Value::Object(map) = value {
            let mut fields = HashMap::new();

            for (k, v) in map {
                match k.as_str() {
                    "ts" | "timestamp" | "time" => {
                        if env.timestamp.is_none() {
                            if let Some(s) = v.as_str() {
                                env.timestamp = Some(s.to_string());
                            }
                        }
                    }
                    "level" | "lvl" => {
                        if env.level.is_none() {
                            if let Some(s) = v.as_str() {
                                env.level = Some(s.to_uppercase());
                            }
                        }
                    }
                    "message" | "msg" => {
                        if env.message.is_none() {
                            if let Some(s) = v.as_str() {
                                env.message = Some(s.to_string());
                            }
                        }
                    }
                    _ => {
                        fields.insert(k, v);
                    }
                }
            }

            if !fields.is_empty() {
                env.fields.extend(fields);
            }
        }
    }

    fn extract_timestamp(&self, text: &str) -> Option<String> {
        self.ts_regex
            .captures(text)
            .and_then(|caps| caps.name("ts").map(|m| m.as_str().to_string()))
    }

    fn extract_level(&self, text: &str) -> Option<String> {
        self.level_regex
            .captures(text)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    }

    /// Strips only the first matched timestamp and level (`replace`, not `replace_all`)
    /// so any subsequent occurrences inside the message body remain intact.
    fn extract_message(&self, text: &str) -> Option<String> {
        let without_ts = self.ts_regex.replace(text, "").trim().to_string();
        let without_level = self.level_regex.replace(&without_ts, "").trim().to_string();

        if without_level.is_empty() {
            None
        } else {
            Some(without_level)
        }
    }
}

use crate::envelope::Envelope;
use crate::line::Line;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

pub struct Parser {
    ts_regex: Regex,
    level_regex: Regex,
}

impl Parser {
    pub fn new() -> Self {
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

    pub fn process(&self, line: Line) -> Value {
        let mut env = Envelope::new(line.number);

        // 1. Try JSON first
        if let Some(json_value) = self.try_json(&line.text) {
            self.extract_from_json(&mut env, json_value);
        }

        // 2. Extract timestamp if missing
        if env.timestamp.is_none() {
            if let Some(ts) = self.extract_timestamp(&line.text) {
                env.timestamp = Some(ts);
            }
        }

        // 3. Extract level if missing
        if env.level.is_none() {
            if let Some(level) = self.extract_level(&line.text) {
                env.level = Some(level);
            }
        }

        // 4. Extract message if missing
        if env.message.is_none() {
            if let Some(msg) = self.extract_message(&line.text) {
                env.message = Some(msg);
            }
        }

        // ❌ REMOVE raw fallback — we no longer use raw
        // ❌ REMOVE env.is_structured() logic entirely

        env.into_json()
    }

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
                        // Keep unknown fields, but do NOT preserve original JSON structure
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

    fn extract_message(&self, text: &str) -> Option<String> {
        // Remove timestamp
        let without_ts = self.ts_regex.replace(text, "").trim().to_string();

        // Remove level
        let without_level = self.level_regex.replace(&without_ts, "").trim().to_string();

        if without_level.is_empty() {
            None
        } else {
            Some(without_level)
        }
    }
}

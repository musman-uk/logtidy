use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Envelope {
    pub line: usize,
    pub timestamp: Option<String>,
    pub level: Option<String>,
    pub message: Option<String>,
    pub fields: HashMap<String, Value>,
    pub raw: Option<String>,
}

impl Envelope {
    pub fn new(line: usize) -> Self {
        Self {
            line,
            ..Default::default()
        }
    }

    pub fn is_structured(&self) -> bool {
        self.timestamp.is_some()
            || self.level.is_some()
            || self.message.is_some()
            || !self.fields.is_empty()
    }

    pub fn into_json(self) -> Value {
        let mut obj = Map::new();
        obj.insert("line".into(), Value::from(self.line));

        if let Some(ts) = self.timestamp {
            obj.insert("timestamp".into(), Value::from(ts));
        }
        if let Some(level) = self.level {
            obj.insert("level".into(), Value::from(level));
        }
        if let Some(msg) = self.message {
            obj.insert("message".into(), Value::from(msg));
        }
        if !self.fields.is_empty() {
            obj.insert("fields".into(), Value::from(self.fields));
        }
        if let Some(raw) = self.raw {
            obj.insert("raw".into(), Value::from(raw));
        }

        Value::Object(obj)
    }
}

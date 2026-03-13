use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Envelope {
    pub line: usize,
    pub timestamp: Option<String>,
    pub level: Option<String>,
    pub message: Option<String>,
    pub fields: HashMap<String, Value>,
}

impl Envelope {
    pub fn new(line: usize) -> Self {
        Self {
            line,
            ..Default::default()
        }
    }

    pub fn into_json(self) -> Value {
        let mut obj = Map::new();

        // 1. Line number (always present)
        obj.insert("line".into(), Value::from(self.line));

        // 2. Message (null if missing)
        obj.insert(
            "message".into(),
            match self.message {
                Some(m) => Value::String(m),
                None => Value::Null,
            },
        );

        // 3. Timestamp (null if missing)
        obj.insert(
            "timestamp".into(),
            match self.timestamp {
                Some(t) => Value::String(t),
                None => Value::Null,
            },
        );

        // 4. Level (null if missing)
        obj.insert(
            "level".into(),
            match self.level {
                Some(l) => Value::String(l),
                None => Value::Null,
            },
        );

        // 5. Fields (null if empty)
        obj.insert(
            "fields".into(),
            if self.fields.is_empty() {
                Value::Null
            } else {
                let mut map = Map::new();
                for (k, v) in self.fields {
                    map.insert(k, v);
                }
                Value::Object(map)
            },
        );

        Value::Object(obj)
    }
}

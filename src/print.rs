use serde_json::Value;

pub fn to_json_preserve_order(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut s = String::from("{");
            let mut first = true;
            for (k, v) in map {
                if !first {
                    s.push(',');
                }
                first = false;
                s.push('"');
                s.push_str(k);
                s.push('"');
                s.push(':');
                s.push_str(&to_json_preserve_order(v));
            }
            s.push('}');
            s
        }
        Value::Array(arr) => {
            let mut s = String::from("[");
            let mut first = true;
            for v in arr {
                if !first {
                    s.push(',');
                }
                first = false;
                s.push_str(&to_json_preserve_order(v));
            }
            s.push(']');
            s
        }
        _ => value.to_string(),
    }
}

use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            _ => OutputFormat::Table,
        }
    }
}

pub struct Formatter;

impl Formatter {
    pub fn format_json_output<T: serde::Serialize>(
        data: &T,
        output_format: OutputFormat,
    ) -> anyhow::Result<String> {
        match output_format {
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(data)?;
                Ok(json)
            }
            OutputFormat::Table => {
                let json_value = serde_json::to_value(data)?;
                Ok(Self::format_as_table(&json_value))
            }
        }
    }

    pub fn format_bytes_output(
        data: &[u8],
        output_format: OutputFormat,
    ) -> anyhow::Result<String> {
        match output_format {
            OutputFormat::Json => {
                let text = String::from_utf8(data.to_vec())?;
                let json_value = serde_json::json!({
                    "content": text,
                    "size_bytes": data.len()
                });
                Ok(serde_json::to_string_pretty(&json_value)?)
            }
            OutputFormat::Table => {
                String::from_utf8(data.to_vec()).map_err(|e| anyhow::anyhow!(e))
            }
        }
    }

    fn format_as_table(value: &Value) -> String {
        match value {
            Value::Array(arr) => Self::format_array_as_table(arr),
            Value::Object(obj) => Self::format_object_as_table(obj),
            _ => value.to_string(),
        }
    }

    fn format_array_as_table(arr: &[Value]) -> String {
        if arr.is_empty() {
            return "(empty)".to_string();
        }

        let mut rows = Vec::new();

        if let Value::Object(first) = &arr[0] {
            let headers: Vec<String> = first.keys().cloned().collect();

            rows.push(headers.join(" | "));
            rows.push("-".repeat(80));

            for item in arr {
                if let Value::Object(obj) = item {
                    let values: Vec<String> = headers
                        .iter()
                        .map(|h| {
                            obj.get(h)
                                .map(|v| format_value(v))
                                .unwrap_or_else(|| "-".to_string())
                        })
                        .collect();
                    rows.push(values.join(" | "));
                }
            }
        }

        rows.join("\n")
    }

    fn format_object_as_table(obj: &serde_json::Map<String, Value>) -> String {
        let mut rows = Vec::new();
        let map: BTreeMap<&String, &Value> = obj.iter().collect();

        for (key, value) in map {
            rows.push(format!("{}: {}", key, format_value(value)));
        }

        rows.join("\n")
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "-".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            if s.len() > 50 {
                format!("{}...", &s[..47])
            } else {
                s.clone()
            }
        }
        Value::Array(arr) => format!("[{} items]", arr.len()),
        Value::Object(obj) => format!("{{{} fields}}", obj.len()),
    }
}

/// Trait for types that can render themselves as a table row.
pub trait TableDisplay: serde::Serialize {
    fn headers() -> Vec<&'static str>;
    fn row(&self) -> Vec<String>;
}

/// Print a slice of table-displayable items to stdout.
pub fn print<T: TableDisplay>(items: &[T], fmt: OutputFormat) {
    match fmt {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(items).unwrap_or_else(|_| "[]".to_string());
            println!("{}", json);
        }
        OutputFormat::Table => {
            if items.is_empty() {
                println!("(empty)");
                return;
            }
            let headers = T::headers();
            println!("{}", headers.join(" | "));
            println!("{}", "-".repeat(80));
            for item in items {
                println!("{}", item.row().join(" | "));
            }
        }
    }
}

/// Print a single serializable item to stdout.
pub fn print_one<T: serde::Serialize>(item: &T, fmt: OutputFormat) {
    match fmt {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(item).unwrap_or_else(|_| "{}".to_string());
            println!("{}", json);
        }
        OutputFormat::Table => {
            let v = serde_json::to_value(item).unwrap_or(Value::Null);
            println!("{}", Formatter::format_as_table(&v));
        }
    }
}

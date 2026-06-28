use anyhow::Result;
use serde::Serialize;

pub fn render<T, F>(value: &T, json: bool, table_renderer: F) -> Result<String>
where
    T: Serialize,
    F: FnOnce(&T) -> String,
{
    if json {
        Ok(serde_json::to_string_pretty(value)?)
    } else {
        Ok(table_renderer(value))
    }
}

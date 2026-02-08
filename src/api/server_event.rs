use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent<'a> {
    ConsoleLine { line: &'a String },
}

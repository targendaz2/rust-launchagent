use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub enum StringOrInt {
    Integer(u32),
    String(String),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum StringOrArrayOfStrings {
    String(String),
    Array(Vec<String>),
}

pub fn default_false() -> bool {
    false
}

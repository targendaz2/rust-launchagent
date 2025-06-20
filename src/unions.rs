use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringOrF32 {
    String(String),
    Integer(f32),
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringOrU32 {
    String(String),
    Integer(u32),
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

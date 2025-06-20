use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum SessionType {
    Single(String),
    Many(Vec<String>),
}

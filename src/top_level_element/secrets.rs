use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum Secrets {
    File { file: String },
    Environment { environment: String },
}

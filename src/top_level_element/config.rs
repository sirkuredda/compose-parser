use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum Config {
    File{
        file: String,
    },
    Environment{
        environment: String,
    },
    Content{
        content: String,
    },
    External{
        external: bool,
        name: Option<String>,
    },
}

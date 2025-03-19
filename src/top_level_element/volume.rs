use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use super::service::deserialize_key_value_or_map;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Volume {
    pub driver: Option<String>,
    pub driver_opts: Option<HashMap<String, serde_yml::Value>>,
    pub external: Option<bool>,
    #[serde(deserialize_with = "deserialize_labels", default)]
    pub labels: Option<HashMap<String, String>>,
    pub name: Option<String>,
}

fn deserialize_labels<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_key_value_or_map(deserializer)
}


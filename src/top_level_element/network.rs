use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use super::service::deserialize_key_value_or_map;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Network {
    pub driver: Option<String>,
    pub driver_opts: Option<HashMap<String, serde_yml::Value>>,
    pub attachable: Option<bool>,
    pub enable_ipv4: Option<bool>,
    pub enable_ipv6: Option<bool>,
    pub external: Option<bool>,
    pub ipam: Option<Ipam>,
    pub internal: Option<bool>,
    #[serde(deserialize_with = "deserialize_network_labels", default)]
    pub labels: Option<HashMap<String, String>>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Ipam {
    pub driver: Option<String>,
    pub config: Option<Vec<IpamConfig>>,
    pub options: Option<HashMap<String, serde_yml::Value>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct IpamConfig {
    pub subnet: Option<String>,
    pub ip_range: Option<String>,
    pub gateway: Option<String>,
    pub aux_addresses: Option<HashMap<String, serde_yml::Value>>,
}

fn deserialize_network_labels<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_key_value_or_map(deserializer)
}

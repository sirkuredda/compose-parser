use serde::{Deserialize, Deserializer, de};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Service {
    pub image: String,
    #[serde(deserialize_with = "deserialize_annotations", default)]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(default = "default_true")]
    pub attach: bool,
    pub blkio_config: Option<BlkioConfig>,
    pub cpu_count: Option<u16>,
    pub cpu_percent: Option<u16>,
    pub cpu_shares: Option<usize>,
    pub cpu_period: Option<String>,
    pub cpu_quota: Option<usize>,
    pub cpu_rt_runtime: Option<String>,
    pub cpu_rt_period: Option<String>,
    pub cpus: Option<f64>,
    pub cpuset: Option<CpuSet>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    pub cgroup: Option<Cgroup>,
    pub cgroup_parent: Option<String>,
    pub command: Option<String>,
    pub configs: Option<ServiceConfigVariant>,
    pub container_name: Option<String>,
    pub credential_spec: Option<CredentialSpec>,
    pub depends_on: Option<DependsOn>,
    // todo pub deploy
    // todo pub develop
    pub device_cgroup_rules: Option<Vec<String>>,
    pub devices: Option<Vec<String>>, // todo: create custom Struct?
    pub dns: Option<SingleOrList>,
    pub dns_opt: Option<Vec<String>>,
    pub dns_search: Option<SingleOrList>,
    pub domainname: Option<String>,
    pub driver_opts: Option<HashMap<String, String>>,
    pub entrypoint: Option<SingleOrList>,
    pub env_file: Option<EnvService>,
    #[serde(deserialize_with = "deserialize_environment", default)]
    pub environment: Option<HashMap<String, String>>,
    pub expose: Option<Vec<String>>,
    pub extends: Option<ExtendsService>,
    pub external_links: Option<Vec<String>>,
    pub extra_hosts: Option<Vec<String>>,
    pub gpu: Option<Vec<GpuData>>, //todo: implement the all syntax
    pub group_add: Option<Vec<String>>,
    // pub healthcheck: todo!(),
    pub hostname: Option<String>,
    pub init: Option<bool>,
    pub ipc: Option<String>,
    pub isolation: Option<String>,
    pub labels: Option<HashmapOrList>,
    pub label_file: Option<SingleOrList>,
    pub links: Option<Vec<String>>,
    pub logging: Option<LoggingService>,
    pub mac_address: Option<String>,
    pub mem_limit: Option<String>,
    pub mem_reservation: Option<String>,
    pub mem_swappiness: Option<u16>,
    pub memswap_limit: Option<String>,
    pub network_mode: Option<String>, //todo: implement enum
    pub networks: Option<Vec<String>>, //todo: implement aliases
    pub oom_kill_disable: Option<String>,
    pub oom_score_adj: Option<i16>,
    pub pid: Option<String>,
    pub pids_limit: Option<usize>,
    pub platform: Option<String>,
    pub ports: Option<Vec<String>>,
    pub post_start: Option<Vec<PostStart>>,
    pub pre_stop: Option<Vec<PostStart>>,
    pub privileged: Option<bool>,
    pub profiles: Option<Vec<String>>,
    pub pull_policy: Option<PullPolicy>,
    pub read_only: Option<bool>,
    pub restart: Option<RestartPolicy>,
    pub runtime: Option<String>,
    pub scale: Option<u16>,
    pub secrets: Option<SecretsSyntax>,
    pub security_opt: Option<Vec<String>>,
    pub shm_size: Option<String>,
    pub stdin_open: Option<bool>,
    pub stop_grace_period: Option<String>,
    pub stop_signal: Option<String>,
    pub storage_opt: Option<HashMap<String, String>>,
    pub sysctls: Option<SysctlsConfig>,
    pub tmpfs: Option<Vec<String>>, //todo: improve
    pub tty: Option<bool>,
    pub ulimits: Option<UlimitsOptions>,
    pub user: Option<String>,
    pub userns_mode: Option<String>,
    pub host: Option<String>,
    pub volumes: Option<Vec<String>>,
    pub volumes_from: Option<Vec<String>>,
    pub working_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BlkioConfig {
    weight: Option<u16>,
    weight_device: Option<Vec<DeviceWeight>>,
    device_read_bps: Option<Vec<DeviceLimit>>,
    device_read_iops: Option<Vec<DeviceLimit>>,
    device_write_bps: Option<Vec<DeviceLimit>>,
    device_write_iops: Option<Vec<DeviceLimit>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DeviceWeight{
    path: String,
    weight: u16
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DeviceLimit {
    path: String,
    rate: String //todo: needs to a number
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CpuSet{
    Range(u32, u32),
    List(Vec<u32>)
}

impl<'de> Deserialize<'de> for CpuSet{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        if let Some((start, end)) = s.split_once('-'){
            let start = u32::from_str(start).map_err(de::Error::custom)?;
            let end = u32::from_str(end).map_err(de::Error::custom)?;
            Ok(CpuSet::Range(start, end))
        }
        else {
            let list: std::result::Result<Vec<u32>, D::Error> = s
                .split(',')
                .map(|item| { u32::from_str(item)}.map_err(de::Error::custom))
                .collect();

            Ok(CpuSet::List(list?))
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Cgroup {
    Host,
    Private
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ServiceConfig {
    source: String,
    target: String,
    uid: String,
    gid: String,
    mode: String
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum ServiceConfigVariant {
    SimpleConfig(Vec<String>),
    ComplexConfig(Vec<ServiceConfig>)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum CredentialSpec {
    File { file: String },
    Registry { registry: String },
    Config { config: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum DependsOnCondition{
    ServiceStarted,
    ServiceHealthy,
    ServiceCompletedSuccessfully
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DependsLongSyntax{
    #[serde(default)]
    restart: bool,
    condition: DependsOnCondition,
    #[serde(default = "default_true")]
    required: bool
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum DependsOn {
    ShortSyntax(Vec<String>),
    LongSyntax(HashMap<String, DependsLongSyntax>)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum SingleOrList {
    Single(String),
    List(Vec<String>)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum HashmapOrList{
    Hashmap(HashMap<String, String>),
    List(Vec<String>)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum SysctlsConfig{
    Hashmap(HashMap<String, i32>),
    List(Vec<String>)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ComplexEnv {
    path: String,
    #[serde(default = "default_true")]
    required: bool,
    format: Option<String>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum EnvService {
    Simple(String),
    List(Vec<String>),
    Complex(Vec<ComplexEnv>)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ExtendsService {
    file: Option<String>,
    service: String
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GpuData{
    driver: String,
    count: u16
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LoggingService{
    driver: String,
    options: Option<HashMap<String, String>>
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PostStartService {
    command: String,
    user: Option<String>,
    privileged: Option<String>
}


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(untagged)]
pub enum UlimitsOptions{
    Nproc{
        nproc: u64
    },
    Nofile{
        nofile: UlimitsNofile
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UlimitsNofile {
    soft: u64,
    hard: u64
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PostStart {
    command: String,
    user: Option<String>,
    privileged: bool,
    environment: Option<Vec<String>>
}

//todo: implement the every_<duration>
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "snake_case")]
pub enum PullPolicy{
    Always,
    Never,
    Missing,
    Build,
    Daily,
    Weekly,
}

#[derive(Debug)]
pub enum RestartPolicy{
    No,
    Always,
    OnFailure(Option<i32>),
    UnlessStopped,
}

impl<'de> Deserialize<'de> for RestartPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct RestartPolicyVisitor;

        impl<'de> de::Visitor<'de> for RestartPolicyVisitor{
            type Value = RestartPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "a string like 'no', 'always', 'on-failure', 'on-failure:N', or 'unless-stopped'",)
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error, 
            {
                match v {
                    "no" => Ok(RestartPolicy::No),
                    "always" => Ok(RestartPolicy::Always),
                    "unless-stopped" => Ok(RestartPolicy::UnlessStopped),
                    s if s.starts_with("on-failure") => {
                        if s == "on-failure" {
                            Ok(RestartPolicy::OnFailure(None))
                        } else if let Some(retries_str) = s.strip_prefix("on-failure:") {
                            match i32::from_str(retries_str) {
                                Ok(num) => Ok(RestartPolicy::OnFailure(Some(num))),
                                Err(_) => Err(de::Error::invalid_value(
                                            de::Unexpected::Str(retries_str),
                                            &"'no', 'always', 'on-failure', 'on-failure:N', or 'unless-stopped'",
                            ))}
                        }
                        else {
                            Err(de::Error::invalid_value(
                                    de::Unexpected::Str(s),
                                    &"'no', 'always', 'on-failure', 'on-failure:N', or 'unless-stopped'",
                            ))
                        }
                    }
                    _ => {
                            Err(de::Error::invalid_value(
                                    de::Unexpected::Str(v),
                                    &"'no', 'always', 'on-failure', 'on-failure:N', or 'unless-stopped'",
                            ))
                    }

                }
            }
        }

        deserializer.deserialize_str(RestartPolicyVisitor)
    }
}

//todo: improve field deserialization
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SecretConfig{
    source: String,
    target: Option<String>,
    uid: Option<String>,
    gid: Option<String>,
    mode: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum SecretsSyntax{
    ShortSyntax(Vec<String>),
    LongSyntax(Vec<SecretConfig>)
}


fn default_true() -> bool { true }

impl Service {
    fn try_get_ports(&self) -> String {
        let ports = self.ports.as_ref().unwrap();

        ports
            .first()
            .map(|port| {
                port.to_string()
                    .split(':')
                    .nth(0)
                    .unwrap_or(port)
                    .to_string()
            })
            .unwrap()
    }
}


pub fn deserialize_key_value_or_map<'de, D, T>(
    deserializer: D,
) -> std::result::Result<Option<HashMap<String, T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + From<String>
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Helper<U> {
        List(Vec<String>),
        Map(HashMap<String, U>),
    }

    match Helper::<T>::deserialize(deserializer) {
        Ok(data) => match data {
            Helper::List(data_list) => {
                let mut map: HashMap<String, T> = HashMap::new();
                for item in data_list {
                    let parts: Vec<&str> = item.splitn(2, '=').collect();
                    match parts.len() {
                        1 => {
                            map.insert(parts[0].to_string(), T::from(String::from("")));
                        }
                        2 => {
                            map.insert(parts[0].to_string(), T::from(parts[1].to_string()));
                        }
                        _ => {
                            return Err(serde::de::Error::custom(format!(
                                "Invalid key=value format: {}",
                                item
                            )))
                        }
                    }
                }

                Ok(Some(map))
            }
            Helper::Map(label_map) => Ok(Some(label_map)),
        },
        Err(err) => Err(err),
    }
}


fn deserialize_annotations<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_key_value_or_map(deserializer)
}

fn deserialize_environment<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_key_value_or_map(deserializer)
}

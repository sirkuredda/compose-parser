use std::path::Path;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::top_level_element::Secrets;
use crate::top_level_element::Network;
use crate::top_level_element::Config;
use crate::top_level_element::Volume;
use crate::top_level_element::Service;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DockerCompose {
    pub version: String,
    pub services: HashMap<String, Service>,
    pub networks: HashMap<String, Network>,
    pub volumes: HashMap<String, Volume>,
    pub configs: HashMap<String, Config>,
    pub secrets: HashMap<String, Secrets>,
}

#[allow(dead_code)]
impl DockerCompose {
    pub fn try_load_compose(path: impl AsRef<Path>) -> Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        serde_yml::from_str(contents.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_deserialization_docker_compose() {
        let mut current_dir = current_dir().unwrap();
        current_dir.push("docker-compose-example.yml");
        let mut file = File::open(current_dir).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
    }

    #[test]
    fn test_load_docker_compose() {
        let mut current_dir = current_dir().unwrap();
        current_dir.push("docker-compose-example.yml");

        let compose = DockerCompose::try_load_compose(&current_dir);

        println!("{:#?}", compose);
    }

}

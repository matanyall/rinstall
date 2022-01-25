#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

static DEBUG: bool = false;
static DRY_RUN: bool = false;

static ALL_MANAGERS: [&str; 2] = ["apt", "snap"];

mod package_managers;
use package_managers::{apt, snap};


pub mod apply {
    use yaml_rust::Yaml;

    pub fn install(yaml: &Yaml) {
        for (service, packages) in yaml["managers"].as_hash().unwrap().iter() {
            let service = service.as_str().unwrap();
            match service {
                "apt" => {
                    crate::apt::install(packages);
                }
                "snap" => {
                    crate::snap::install(packages);
                }
                _ => {
                    println!("{}", service);
                }
            }
        }
    }
}

pub mod capture {
    use std::fs::File;
    use std::io::prelude::*;
    // use crate::YamlMap;
    use serde_yaml::{Value};
    use serde_yaml::mapping::{Mapping};

    pub fn capture(yaml_output: String, managers: Vec<&str>) {
        let mut managers = managers;

        if managers.len() == 0 || managers.contains(&"all") {
            managers = crate::ALL_MANAGERS.to_vec();
        }

        let mut yaml_map = Mapping::new();
        yaml_map.insert(Value::String("managers".into()), Value::Mapping(Mapping::new()));

        for service in managers {
            match service {
                "apt" => {
                    yaml_map
                        .get_mut(&Value::String("managers".into()))
                        .unwrap()
                        .as_mapping_mut()
                        .unwrap()
                        .insert(Value::String("apt".to_string()), crate::apt::capture());
                }
                "snap" => {
                    yaml_map
                        .get_mut(&Value::String("managers".into()))
                        .unwrap()
                        .as_mapping_mut()
                        .unwrap()
                        .insert(Value::String("snap".to_string()), crate::snap::capture());
                }
                _ => {
                    println!("{}", service);
                }
            }
        }


        let output_string = serde_yaml::to_string(&yaml_map).unwrap();

        // write output_string to file called yaml_output
        let mut file = File::create(yaml_output).unwrap();
        file.write_all(output_string.as_bytes()).unwrap();
    }
}

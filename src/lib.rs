#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

static DEBUG: bool = true;
static DRY_RUN: bool = false;

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
    use yaml_rust::{Yaml, YamlEmitter};
    use linked_hash_map::LinkedHashMap;
    use serde_yaml;
    use std::collections::HashMap;

    pub fn capture(yaml_output: String, managers: Vec<&str>) {

        println!("{:?}", managers);
        println!("{:?}", yaml_output);

        let mut output_string = String::new();
        output_string.push_str(&format!("---\n"));
        output_string.push_str(&format!("managers:\n"));
        let mut output_map = HashMap::new(); 
        output_map.insert("managers".to_string(), HashMap::new());
        // output_yaml.insert(Yaml::from_str("managers"), Yaml::Hash(LinkedHashMap::<Yaml, Yaml>::new()));

        // {
        //     "managers" => Yaml::Hash(linked_hash_map! {
        //         "apt" => Yaml::Hash(linked_hash_map! {}),
        //         "snap" => Yaml::Hash(linked_hash_map! {}),
        //     }),
        // };
        for service in managers{
            // let service = service.as_str().unwrap();
            match service {
                "apt" => {

                    output_map.get_mut("managers").unwrap().insert("apt".to_string(), crate::apt::capture());
                    // let apt_yaml = Yaml::Hash(crate::apt::capture());
                    // output_yaml[&Yaml::from_str("managers")].as_hash().unwrap().insert(Yaml::from_str("apt"), apt_yaml);
                    // output_string.push_str(apt_yaml);
                }
                "snap" => {
                    // crate::snap::install(packages);
                }
                _ => {
                    println!("{}", service);
                }
            }
        }

        let output_string = serde_yaml::to_string(&output_map).unwrap();
        println!("{:?}", output_string);

        // let output = YamlEmitter::new(&mut output_string);
    }
}

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

pub mod capture {}

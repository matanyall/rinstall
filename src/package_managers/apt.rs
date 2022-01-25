pub use self::apt::{install, capture};

mod apt {
    use std::process::Command;
    use yaml_rust::{Yaml};
    use serde_yaml::{Value};
    use serde_yaml::mapping::{Mapping};

    pub fn install(modules: &Yaml) {
        let repository_list = match modules["repositories"].as_vec() {
            Some(x) => x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>(),
            None => vec![],
        };

        let utilities_list = match modules["utilities"].as_vec() {
            Some(x) => x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>(),
            None => vec![],
        };

        let package_list = match modules["packages"].as_vec() {
            Some(x) => x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>(),
            None => vec![],
        };
        install_repositories(repository_list);
        install_utilities(utilities_list);
        install_packages(package_list);
    }

    fn install_repositories(repository_list: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", repository_list);
        }
        if repository_list.len() > 0 {
            for repository in repository_list {
                if crate::DRY_RUN {
                    println!("apt-add-repository {}", repository);
                } else {
                    let output = Command::new("apt-add-repository")
                        .arg("-y")
                        .arg(repository)
                        .output()
                        .expect("failed to execute process");
                    if crate::DEBUG {
                        println!("{}", String::from_utf8_lossy(&output.stderr));
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                }
            }

            apt_update();
        }
    }

    fn install_packages(package_list: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", package_list);
        }
        if package_list.len() > 0 {
            if crate::DRY_RUN {
                let output = Command::new("apt")
                    .arg("install")
                    .arg("-y")
                    .arg("--dry-run")
                    .args(package_list)
                    .output()
                    .expect("failed to execute process");
                if crate::DEBUG {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            } else {
                let output = Command::new("apt")
                    .arg("install")
                    .arg("-y")
                    .args(package_list)
                    .output()
                    .expect("failed to execute process");
                if crate::DEBUG {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
        }
    }

    fn install_utilities(utilities_list: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", utilities_list);
        }
        if utilities_list.len() > 0 {
            if crate::DRY_RUN {
                let output = Command::new("apt")
                    .arg("install")
                    .arg("--dry-run")
                    .arg("-y")
                    .args(utilities_list)
                    .output()
                    .expect("failed to execute process");
                if crate::DEBUG {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }            
            } else {
                let output = Command::new("apt")
                    .arg("install")
                    .arg("-y")
                    .args(utilities_list)
                    .output()
                    .expect("failed to execute process");
                if crate::DEBUG {
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            apt_update();
        }
    }

    /// apt update
    /// 
    /// This function will update the apt cache.
    /// 
    /// # Examples
    fn apt_update() {

        if crate::DRY_RUN {
            println!("apt-get update");
        } else {
            let output = Command::new("apt")
                .arg("update")
                .output()
                .expect("failed to execute process");
            if crate::DEBUG {
                println!("{}", String::from_utf8_lossy(&output.stderr));
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
    }

    /// Capture the output of the command
    /// 
    /// # Arguments
    /// 
    /// 
    /// 
    pub fn capture() -> Value {
        
        let mut output_map = Mapping::new();

        output_map.insert(Value::String("repositories".to_string()), capture_repositories());

        output_map.insert(Value::String("utilities".to_string()), capture_utilities());

        output_map.insert(Value::String("packages".to_string()), capture_packages());

        Value::Mapping(output_map)

    }
    // TODO: make repo capture
    #[allow(unused_mut)]
    fn capture_repositories() -> Value {

        let mut repo_list = vec![];

        Value::Sequence(repo_list)

    }

    #[allow(unused_mut)]
    fn capture_utilities() -> Value {

        let mut utility_list = vec![];

        Value::Sequence(utility_list)


    }

    fn capture_packages() -> Value {
        let output = Command::new("apt-mark")
            .arg("showmanual")
            .output()
            .expect("failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let packages = stdout.split("\n");
        let mut package_list = vec![];
        for package in packages {
            if package.len() > 0 {
                package_list.push(Value::String(package.to_string()));
            }
        }
        Value::Sequence(package_list)

    }
}

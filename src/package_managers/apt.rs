pub use self::apt::{install, capture};

mod apt {
    use std::process::Command;
    use yaml_rust::{Yaml};
    // use std::collections::HashMap;
    use std::collections::BTreeMap;

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

    pub fn capture() -> BTreeMap<String, Vec<String>> {
        
        let mut output_map = BTreeMap::new();

        output_map.insert("repositories".to_string(), capture_repositories());

        output_map.insert("utilities".to_string(), capture_utilities());

        output_map.insert("packages".to_string(), capture_packages());

        output_map

    }
    // TODO: make repo capture
    #[allow(unused_mut)]
    fn capture_repositories() -> Vec<String> {

        let mut repo_list = vec![];

        repo_list

    }

    #[allow(unused_mut)]
    fn capture_utilities() -> Vec<String> {

        let mut utility_list = vec![];

        utility_list


    }

    fn capture_packages() -> Vec<String> {
        let output = Command::new("apt-mark")
            .arg("showmanual")
            .output()
            .expect("failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let packages = stdout.split("\n");
        let mut package_list = vec![];
        for package in packages {
            if package.len() > 0 {
                package_list.push(package.to_string());
            }
        }
        package_list

    }
}

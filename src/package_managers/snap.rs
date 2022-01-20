pub use self::snap::install;

mod snap {
    use std::process::Command;
    use yaml_rust::Yaml;

    pub fn install(modules: &Yaml) {
        let classic_package_list = match modules["packages"]["classic"].as_vec() {
            Some(x) => x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>(),
            None => vec![],
        };

        let strict_package_list = match modules["packages"]["strict"].as_vec() {
            Some(x) => x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>(),
            None => vec![],
        };

        let devmode_package_list = match modules["packages"]["devmode"].as_vec() {
            Some(x) => x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>(),
            None => vec![],
        };

        install_classic(classic_package_list);
        install_strict(strict_package_list);
        install_devmode(devmode_package_list);
    }

    fn install_classic(package_list: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", package_list);
        }
        if package_list.len() > 0 {
            if crate::DRY_RUN {
                println!("snap install {}", package_list.join(" "));
            } else {
                let output = Command::new("snap")
                    .arg("install")
                    .arg("--classic")
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

    fn install_strict(package_list: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", package_list);
        }
        if package_list.len() > 0 {
            if crate::DRY_RUN {
                println!("snap install {}", package_list.join(" "));
            } else {
                let output = Command::new("snap")
                    .arg("install")
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

    fn install_devmode(package_list: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", package_list);
        }
        if package_list.len() > 0 {
            if crate::DRY_RUN {
                println!("snap install {}", package_list.join(" "));
            } else {
                let output = Command::new("snap")
                    .arg("install")
                    .arg("--devmode")
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
}

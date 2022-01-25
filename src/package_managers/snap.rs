pub use self::snap::capture;
pub use self::snap::install;

mod snap {
    use serde_yaml::mapping::Mapping;
    use serde_yaml::Value;
    use std::process::Command;
    use yaml_rust::Yaml;

    pub fn install(modules: &Yaml) {
        let classic_package_list = match modules["classic"].as_vec() {
            Some(x) => x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>(),
            None => vec![],
        };

        let strict_package_list = match modules["strict"].as_vec() {
            Some(x) => x.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>(),
            None => vec![],
        };

        let devmode_package_list = match modules["devmode"].as_vec() {
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

    pub fn capture() -> Value {
        let mut output_map = Mapping::new();

        output_map.insert(Value::String("classic".into()), capture_classic());

        output_map.insert(Value::String("strict".into()), capture_strict());

        output_map.insert(Value::String("devmode".into()), capture_devmode());
        Value::Mapping(output_map)
    }


    fn capture_classic() -> Value {
        let output = Command::new("snap")
            .arg("list")
            .arg("--color=never")
            .arg("--unicode=never")
            .output()
            .expect("failed to execute process");

        let mut output_vec = Vec::new();
        let cmd_output_string = String::from_utf8_lossy(&output.stdout);
        let cmd_output_vec = cmd_output_string.split("\n").collect::<Vec<&str>>();
        for line in cmd_output_vec[1..].iter() {
            if line.contains(" ") {
                let line_vec = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let package_name = line_vec[0];
                let mode = line_vec[5];
                if crate::DEBUG {
                    println!("{:?}", line_vec);
                    println!("package_name: {}", package_name);
                    println!("mode: {}", mode);
                }
                if mode == "classic" {
                    output_vec.push(Value::String(package_name.to_string()));
                }
            }
        }
        Value::Sequence(output_vec)
    }

    fn capture_strict() -> Value {
        let output = Command::new("snap")
            .arg("list")
            .arg("--color=never")
            .arg("--unicode=never")
            .output()
            .expect("failed to execute process");

        let mut output_vec = Vec::new();
        let cmd_output_string = String::from_utf8_lossy(&output.stdout);
        let cmd_output_vec = cmd_output_string.split("\n").collect::<Vec<&str>>();
        for line in cmd_output_vec[1..].iter() {
            if line.contains(" ") {
                let line_vec = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let package_name = line_vec[0];
                let mode = line_vec[5];
                if crate::DEBUG {
                    println!("{:?}", line_vec);
                    println!("package_name: {}", package_name);
                    println!("mode: {}", mode);
                }
                if mode == "-" {
                    output_vec.push(Value::String(package_name.to_string()));
                }
            }
        }
        Value::Sequence(output_vec)
    }

    fn capture_devmode() -> Value {
        let output = Command::new("snap")
            .arg("list")
            .arg("--color=never")
            .arg("--unicode=never")
            .output()
            .expect("failed to execute process");

        let mut output_vec = Vec::new();
        let cmd_output_string = String::from_utf8_lossy(&output.stdout);
        let cmd_output_vec = cmd_output_string.split("\n").collect::<Vec<&str>>();
        for line in cmd_output_vec[1..].iter() {
            if line.contains(" ") {
                let line_vec = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let package_name = line_vec[0];
                let mode = line_vec[5];
                if crate::DEBUG {
                    println!("{:?}", line_vec);
                    println!("package_name: {}", package_name);
                    println!("mode: {}", mode);
                }
                if mode == "devmode" {
                    output_vec.push(Value::String(package_name.to_string()));
                }
            }
        }
        Value::Sequence(output_vec)
    }
}

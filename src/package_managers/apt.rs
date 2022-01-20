
pub use self::apt::install_packages;


mod apt {
    use std::process::Command;
    use yaml_rust::Yaml;

    pub fn install_packages(packages: &Yaml) {
        let utilities_list = packages["utilities"].as_vec().unwrap();
        let utilities_list = utilities_list
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<&str>>();

        let standard_list = packages["standard"].as_vec().unwrap();
        let standard_list = standard_list
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<&str>>();

        install_utilities(utilities_list);
        install_standard(standard_list);
    }
    fn install_utilities(utilities: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", utilities);
        }
        let output = Command::new("apt")
            .arg("install")
            .arg("--dry-run")
            .args(utilities)
            .output()
            .expect("failed to execute process");
        if crate::DEBUG {
            println!("{}", String::from_utf8_lossy(&output.stderr));
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
    fn install_standard(standard: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", standard);
        }
        let output = Command::new("apt")
            .arg("install")
            .arg("--dry-run")
            .args(standard)
            .output()
            .expect("failed to execute process");
        if crate::DEBUG {
            println!("{}", String::from_utf8_lossy(&output.stderr));
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}

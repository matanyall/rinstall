pub use self::snap::install_packages;

mod snap {
    use std::process::Command;
    use yaml_rust::Yaml;
    // use std::env;

    pub fn install_packages(packages: &Yaml) {
        let packages_list = packages.as_vec().unwrap();
        let packages_list = packages_list
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<&str>>();

        install(packages_list);
    }

    fn install(packages: Vec<&str>) {
        if crate::DEBUG {
            println!("{:?}", packages);
        }

        let output = Command::new("snap")
            .arg("install")
            .args(packages)
            .output()
            .expect("failed to execute process");
        if crate::DEBUG {
            println!("{}", String::from_utf8_lossy(&output.stderr));
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}

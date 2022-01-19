#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

static DEBUG: bool = true;


pub mod apt_packages {
    use std::process::Command;
    use yaml_rust::{Yaml};

    

    pub fn install_packages(packages: &Yaml) {

        let utilities_list = packages["utilities"].as_vec().unwrap();
        let utilities_list = utilities_list.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();

        let standard_list = packages["standard"].as_vec().unwrap();
        let standard_list = standard_list.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();

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

pub mod snap_packages {
    use std::process::Command;
    use yaml_rust::{Yaml};
    // use std::env;


    pub fn install_packages(packages: &Yaml) {

        let packages_list = packages.as_vec().unwrap();
        let packages_list = packages_list.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();

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

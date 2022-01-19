use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

extern crate yaml_rust;
use yaml_rust::{Yaml};
use yaml_rust::YamlLoader;

fn main() {
    println!("Hello, world!");
    // put in help menu and CLI options


    // read in the yaml file
    let file_name = ".test.yml";
    let docs = load_file(file_name);
    let apt_package_list = docs[0]["packages"]["apt"]["utilities"].as_vec().unwrap();
    let apt_package_list = apt_package_list.iter().map(|x| x.as_str().unwrap()).collect::<Vec<&str>>();
    println!("{:?}", apt_package_list);

    // install the packages
    let output = Command::new("apt")
        .arg("install")
        .args(apt_package_list)
        .output()
        .expect("failed to execute process");
    
    println!("{}", String::from_utf8_lossy(&output.stderr));

}

fn load_file(file: &str) -> Vec<Yaml> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    docs

    // iterate / process doc[s] ..
}
use std::fs::File;
use std::io::prelude::*;

extern crate yaml_rust;
use yaml_rust::{Yaml};
use yaml_rust::YamlLoader;


use clap::{App, Arg, arg, AppSettings};


use rinstall::{apt_packages, snap_packages};

fn main() {
    let m = App::new("rinstall")
        .version("0.1.0")
        .author("Matanya Loewenthal");
    // let m = m.arg(Arg::new("config")
    //     .short('c')
    //     .long("config")
    //     .value_name("FILE")
    //     .help("Sets a custom config file")
    //     .takes_value(true));

    let m = m.subcommand(App::new("apply")
            .about("Apply the configuration file")
            .arg(Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true))
            .setting(AppSettings::ArgRequiredElseHelp));

    let matches = m.get_matches();

    println!("{:?}", matches);


    println!("Hello, world!");
    // put in help menu and CLI options


    // read in the yaml file
    let file_name = ".test.yml";
    let docs = load_file(file_name);


    apt_packages::install_packages(&docs[0]["packages"]["apt"]);
    snap_packages::install_packages(&docs[0]["packages"]["snap"]);

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
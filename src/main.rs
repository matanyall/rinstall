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
        // .arg(Arg::new("INPUT")
        //         .required(true)
        //         .help("Apply this input file to your system")
        //         .index(1));
    // let m = m.arg(Arg::new("config")
    //     .short('c')
    //     .long("config")
    //     .value_name("FILE")
    //     .help("Sets a custom config file")
    //     .takes_value(true));

    let apply = App::new("apply")
        .version("0.1.0")
        .author("Matanya Loewenthal")
        .about("Apply this input file to your system")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("INPUT")
                .required(true)
                .help("Apply this input file to your system")
                .index(1));
    
    let capture = App::new("capture")
        .version("0.1.0")
        .author("Matanya Loewenthal")
        .about("Capture your system configuration")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("OUTPUT")
                .required(true)
                .help("Capture your system configuration")
                .index(1));

    let m = m.subcommand(apply);
    let m = m.subcommand(capture);
            

    let matches = m.get_matches();
    match matches.subcommand() {
        Some(("apply", matches)) => {
            let input_file = matches.value_of("INPUT").unwrap();
            let mut f = File::open(input_file).expect("file not found");
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("something went wrong reading the file");
            println!("input file is:\n{}", contents);
            // let yaml = YamlLoader::load_from_str(&contents).unwrap();
            // let yaml = yaml[0].clone();
            // let yaml = yaml.as_hash().unwrap();
            // let yaml = yaml.clone();
            // let yaml = yaml.get("apt").unwrap();
            // let yaml = yaml.as_hash().unwrap();
            // let yaml = yaml.clone();
            // apt_packages::install_packages(&yaml);
            // let yaml = yaml.get("snap").unwrap();
            // let yaml = yaml.as_hash().unwrap();
            // let yaml = yaml.clone();
            // snap_packages::install_packages(&yaml);
        },
        Some(("capture", matches)) => {
            // let output_file = matches.value_of("OUTPUT").unwrap();
            // let mut f = File::create(output_file).expect("file not found");
            // let yaml = YamlLoader::load_from_str("").unwrap();
            // let yaml = yaml[0].clone();
            // let yaml = yaml.as_hash().unwrap();
            // let yaml = yaml.clone();
            // let yaml = yaml.get("apt").unwrap();
            // let yaml = yaml.as_hash().unwrap();
            // let yaml = yaml.clone();
            // apt_packages::install_packages(&yaml);
            // let yaml = yaml.get("snap").unwrap();
            // let yaml = yaml.as_hash().unwrap();
            // let yaml = yaml.clone();
            // snap_packages::install_packages(&yaml);
        },
        _ => {}
    }

    println!("{:?}", matches.value_of("INPUT"));


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

struct Config {
    input_file: String,
    output_file: String,
}

struct CLI {
    config: Config,
}
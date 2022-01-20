use std::fs::File;
use std::io::prelude::*;

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

use rinstall::apply;

mod cli;

fn main() {
    let matches = cli::run();

    match matches.subcommand() {
        Some(("apply", matches)) => {
            let input_file = matches.value_of("INPUT").unwrap();
            let apply_yaml = load_file(input_file);
            let apply_yaml = &apply_yaml[0];
            apply::install(apply_yaml);
        }
        Some(("capture", matches)) => {
            // TODO: capture
        }
        _ => {}
    }
}

fn load_file(file: &str) -> Vec<Yaml> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    docs
}

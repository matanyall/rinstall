use std::fs::File;
use std::io::prelude::*;

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

use rinstall::{apply, capture};

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
            let output_file = matches.value_of("OUTPUT").unwrap();
            let output_file = String::from(output_file);
            let managers = matches.values_of("MANAGER").unwrap_or(clap::Values::default()).collect::<Vec<&str>>();
            capture::capture(output_file, managers);

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

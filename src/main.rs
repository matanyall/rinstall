use std::fs::File;
use std::io::prelude::*;

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

use clap::{App, AppSettings, Arg, ArgMatches};

use rinstall::apply;

fn main() {
    let matches = CLI::run();

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

struct CLI {}

impl CLI {
    fn run() -> ArgMatches {
        let m = App::new("rinstall")
            .version("0.1.0")
            .author("Matanya Loewenthal")
            .about("A tool for reinstalling packages")
            .setting(AppSettings::SubcommandRequired);

        let apply = App::new("apply")
            .version("0.1.0")
            .author("Matanya Loewenthal")
            .about("Apply this input file to your system")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::new("INPUT")
                    .required(true)
                    .help("Apply this input file to your system")
                    .index(1),
            );
        let capture = App::new("capture")
            .version("0.1.0")
            .author("Matanya Loewenthal")
            .about("Capture your system configuration")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::new("OUTPUT")
                    .required(true)
                    .help("Capture your system configuration")
                    .index(1),
            );

        let m = m.subcommand(apply);
        let m = m.subcommand(capture);

        m.get_matches()
    }
}

pub use self::cli::run;

mod cli {
    //! CLI for rinstall
    //! 
    //! Provides for the following subcommands:
    //! - `apply`: Apply a YAML file to the system
    //! - `capture`: Capture the current state of the system
    //! 

    use clap::{App, AppSettings, Arg, ArgMatches};

    /// Runs the CLI
    pub fn run() -> ArgMatches {

        CLI::run()

    }

    /// Struct for the CLI, allows transition to other frameworks in the future
    pub struct CLI {}

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
                )
                .arg(Arg::new("MANAGER")
                    .long("manager")
                    .short('m')
                    .takes_value(true)
                    .default_missing_value("all")
                    .required(false)
                    .multiple_values(true)
                    .help("The package manager to capture")
                );

            let m = m.subcommand(apply);
            let m = m.subcommand(capture);

            m.get_matches()
        }
    }
}


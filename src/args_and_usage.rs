use clap::{Arg, App};
use std::path::PathBuf;

// Programmer defined constants
static PROGRAM_NAME: &'static str = "blockpass";

// Derived constants
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub keep_orig: bool,
}

pub fn parse_args() -> Args {
    let args = App::new(PROGRAM_NAME)
        .version(VERSION)
        .author("Russell W. Bentley <russell.w.bentley@icloud.com>")
        .about("A tool to fix up pandoc html for my website")
        .arg(Arg::with_name("INPUT")
            .help("The file pandoc spits out")
            .long("input")
            .value_name("input/file.ex")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("KEEP_ORIG")
            .help("Defualt behaivor is to delete original file")
            .long("keep-orig"))
        .get_matches();

    let input_path_raw = args.value_of("INPUT").unwrap();
    let input_path = PathBuf::from(input_path_raw);

    Args {
        input: input_path,
        keep_orig: args.is_present("KEEP_ORIG"),
    }
}



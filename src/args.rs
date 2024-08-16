use clap::{App, Arg};

pub struct Args {
    pub path: String,
    pub quiet: bool,
}

impl Args {
    pub fn new() -> Args {
        let matches = App::new("scantron")
            .version("1.0")
            .author("Your Name")
            .about("A CLI tool to scan directories for files")
            .arg(Arg::with_name("path")
                 .short("p")
                 .long("path")
                 .help("Specify the path to scan")
                 .takes_value(true))
            .arg(Arg::with_name("quiet")
                 .short("d")
                 .long("quiet")
                 .help("Don't print the output to the terminal")
                 .takes_value(false))
            .get_matches();

        Args {
            path: matches.value_of("path").unwrap_or(".").to_string(),
            quiet: matches.is_present("quiet"),
        }
    }
}
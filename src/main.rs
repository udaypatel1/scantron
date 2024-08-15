use std::collections::HashMap;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use clap::{App, Arg};

#[derive(Serialize, Deserialize)]
struct FileType {
    files: Vec<String>,
}

fn main() {
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

    let path = matches.value_of("path").unwrap_or(".");
    let quiet = matches.is_present("quiet");

    let mut file_types = HashMap::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let file_type = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let file_type = file_type.to_lowercase().to_string();

        if !file_type.is_empty() {
            file_types.entry(file_type.clone()).or_insert(Vec::new());
            file_types.get_mut(&file_type.clone()).unwrap().push(path.to_string_lossy().to_string());
        }
    }

    let json = serde_json::to_string_pretty(&file_types).unwrap();

    if quiet {
        std::fs::write("scantron.json", json.as_bytes()).unwrap();
    } else {
        println!("{}", json);
    }
}

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct FileType {
    files: Vec<String>,
}

fn main() {
    let mut file_types = HashMap::new();

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let file_type = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let file_type = file_type.to_lowercase().to_string();

        if file_type != "" {
            file_types.entry(file_type.clone()).or_insert(Vec::new());
            file_types.get_mut(&file_type.clone()).unwrap().push(path.to_string_lossy().to_string());
        }
    }

    let json = serde_json::to_string_pretty(&file_types).unwrap();
    println!("{}", json);
}

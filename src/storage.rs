use crate::entry::Entry;
use directories::ProjectDirs;
use std::error::Error;
use std::fs::{OpenOptions, create_dir_all, read_to_string};
use std::io::Write;
use std::path::PathBuf;

fn data_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_dirs =
        ProjectDirs::from("", "", "wokjo").ok_or("We couldn't find the system data directory")?;

    let data_dir = project_dirs.data_dir();

    create_dir_all(data_dir)?;

    Ok(data_dir.join("entries.jsonl"))
}

pub fn save(entry: &Entry) -> Result<(), Box<dyn Error>> {
    let path = data_file_path()?;
    let json = serde_json::to_string(entry)?;

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(file, "{}", json)?;

    Ok(())
}

pub fn load() -> Result<Vec<Entry>, Box<dyn Error>> {
    let path = data_file_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = read_to_string(path)?;

    let entries = content
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<Vec<Entry>, _>>()?;

    Ok(entries)
}

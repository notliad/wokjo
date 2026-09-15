use crate::entry::Entry;
use crate::task::Task;
use directories::ProjectDirs;
use serde::Serialize;
use std::error::Error;
use std::fs::{OpenOptions, create_dir_all, read_to_string};
use std::io::Write;
use std::path::PathBuf;

pub fn data_file_path(filename: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_dirs =
        ProjectDirs::from("", "", "wokjo").ok_or("We couldn't find the system data directory")?;

    let data_dir = project_dirs.data_dir();

    create_dir_all(data_dir)?;

    Ok(data_dir.join(filename))
}

pub fn save<T: Serialize>(item: &T, filename: &str) -> Result<(), Box<dyn Error>> {
    let filename = format!("{}.jsonl", filename);
    let path = data_file_path(&filename)?;
    let json = serde_json::to_string(item)?;

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(file, "{}", json)?;

    Ok(())
}

pub fn load() -> Result<Vec<Entry>, Box<dyn Error>> {
    let path = data_file_path("entries.jsonl")?;

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

pub fn load_task() -> Result<Vec<Task>, Box<dyn Error>> {
    let path = data_file_path("tasks.jsonl")?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = read_to_string(path)?;

    let tasks = content
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<Vec<Task>, _>>()?;

    Ok(tasks)
}

pub fn save_all<T: Serialize>(items: &[T], filename: &str) -> Result<(), Box<dyn Error>> {
    let filename = format!("{}.jsonl", filename);
    let path = data_file_path(&filename)?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    for item in items {
        let json = serde_json::to_string(item)?;

        writeln!(file, "{}", json)?;
    }

    Ok(())
}

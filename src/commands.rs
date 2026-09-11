use crate::entry::Entry;
use crate::storage;
use chrono::{Datelike, Duration, Local, NaiveDate};

pub fn add(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entry = Entry::new(message)?;

    storage::save(&entry)?;

    println!("Entry added!");

    Ok(())
}

fn show_entries_for_date(date: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
    let entries = storage::load()?;
    let mut found = false;

    println!("{}\n----------", date.format("%d/%m/%Y"));

    for entry in entries {
        if entry.timestamp.date_naive() == date {
            println!("{} {}", entry.formatted_time(), entry.message);
            found = true;
        }
    }

    if !found {
        println!("No activity logged.");
    }

    Ok(())
}

pub fn today() -> Result<(), Box<dyn std::error::Error>> {
    let today = Local::now().date_naive();

    show_entries_for_date(today)
}

pub fn yesterday() -> Result<(), Box<dyn std::error::Error>> {
    let yesterday = (Local::now() - Duration::days(1)).date_naive();

    show_entries_for_date(yesterday)
}

pub fn day(date: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
    show_entries_for_date(date)
}

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = storage::load()?;
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    if entries.is_empty() {
        println!("No activity logged.");
        return Ok(());
    }

    let mut current_day: Option<NaiveDate> = None;

    for entry in entries {
        let entry_day = entry.timestamp.date_naive();
        if current_day != Some(entry_day) {
            println!("\n{}\n----------", entry_day.format("%d/%m/%Y"));
            current_day = Some(entry_day);
        }
        println!("{} - {}", entry.formatted_time(), entry.message)
    }

    Ok(())
}

pub fn week() -> Result<(), Box<dyn std::error::Error>> {
    let today = Local::now().date_naive();

    let days_from_monday = today.weekday().number_from_monday() - 1;
    let monday = today - Duration::days(days_from_monday.into());

    let mut entries = storage::load()?;
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let mut current_day: Option<NaiveDate> = None;

    for entry in entries {
        let entry_day = entry.timestamp.date_naive();

        if entry_day < monday || entry_day > today {
            continue;
        }

        if current_day != Some(entry_day) {
            println!("\n{}\n----------", entry_day.format("%d/%m/%Y"));
            current_day = Some(entry_day);
        }
        println!("{} - {}", entry.formatted_time(), entry.message)
    }

    Ok(())
}

fn print_entries(entries: &Vec<Entry>) {
    let mut current_day: Option<NaiveDate> = None;

    for entry in entries {
        let entry_day = entry.timestamp.date_naive();
        if current_day != Some(entry_day) {
            println!("\n{}\n----------", entry_day.format("%d/%m/%Y"));
            current_day = Some(entry_day);
        }
        println!("{} - {}", entry.formatted_time(), entry.message)
    }
}
pub fn search(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = storage::load()?;
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let lower_query = query.to_lowercase();

    let filtered_entries: Vec<Entry> = entries
        .into_iter()
        .filter(|entry| entry.message.to_lowercase().contains(&lower_query))
        .collect();

    if filtered_entries.is_empty() {
        println!("No activities found for {}", query);
    }

    print_entries(&filtered_entries);

    Ok(())
}

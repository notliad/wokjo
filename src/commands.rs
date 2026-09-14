use crate::storage;
use crate::task::Task;
use crate::{commands, entry::Entry};
use chrono::{Datelike, Duration, Local, NaiveDate};

pub fn add(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let today = Local::now().date_naive();
    let entry = Entry::new(message)?;
    storage::save(&entry, "entries")?;

    println!("Entry added!");

    show_entries_for_date(today)?;

    Ok(())
}

pub fn task_add(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let task = Task::new(message)?;
    storage::save(&task, "tasks")?;

    println!("Task added!\n");
    task_todo()?;

    Ok(())
}

fn show_entries_for_date(date: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
    let entries = storage::load()?;

    let filtered_entries: Vec<Entry> = entries
        .into_iter()
        .filter(|entry| entry.timestamp.date_naive() == date)
        .collect();

    if filtered_entries.is_empty() {
        println!("No activity logged.");
    }

    print_entries(&filtered_entries);

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

    print_entries(&entries);

    Ok(())
}

pub fn task_list() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = storage::load_task()?;

    if tasks.is_empty() {
        println!("No tasks logged.");
        return Ok(());
    }

    print_tasks(&tasks);

    Ok(())
}

pub fn task_check(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = storage::load_task()?;
    let task = tasks.iter_mut().find(|task| task.id == id);

    match task {
        Some(task) => {
            task.done = !task.done;
            if task.done {
                println!("Task checked!");
            } else {
                println!("Task unchecked!");
            }
        }
        None => {
            return Err(format!("No task found with ID \"{}\".", id).into());
        }
    }

    storage::save_all(&tasks, "tasks")
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
            println!("No activity logged this week!");
            continue;
        }

        if current_day != Some(entry_day) {
            println!("\n{}\n----------", entry_day.format("%d/%m/%Y"));
            current_day = Some(entry_day);
        }
        println!(
            "{} {} - {}",
            entry.id,
            entry.formatted_time(),
            entry.message
        )
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
        println!(
            "{} {} - {}",
            entry.id,
            entry.formatted_time(),
            entry.message
        )
    }
}

fn print_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        let status = if task.done { "[x]" } else { "[ ] " };
        println!("{} {} - {}", status, task.id, task.message);
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

pub fn delete(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entries = storage::load()?;
    let original_len = entries.len();

    let filtered_entries: Vec<Entry> = entries.into_iter().filter(|entry| entry.id != id).collect();

    if original_len == filtered_entries.len() {
        return Err(format!("No entry found with ID \"{}\".", id).into());
    }

    storage::save_all(&filtered_entries, "entries")?;
    println!("Log deleted!");

    Ok(())
}

pub fn edit(id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = storage::load()?;
    let entry = entries.iter_mut().find(|entry| entry.id == id);

    match entry {
        Some(entry) => {
            entry.message = message.to_string();
            println!("Log edited!");
        }
        None => {
            return Err(format!("No entry found with ID \"{}\".", id).into());
        }
    }

    storage::save_all(&entries, "entries")
}

pub fn task_delete(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = storage::load_task()?;
    let original_len = tasks.len();

    let filtered_tasks: Vec<Task> = tasks.into_iter().filter(|task| task.id != id).collect();

    if original_len == filtered_tasks.len() {
        return Err(format!("No task found with ID \"{}\".", id).into());
    }

    storage::save_all(&filtered_tasks, "tasks")?;
    println!("Task deleted!");

    Ok(())
}

pub fn task_edit(id: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = storage::load_task()?;
    let task = tasks.iter_mut().find(|task| task.id == id);

    match task {
        Some(task) => {
            task.message = message.to_string();
            println!("Task edited!");
        }
        None => {
            return Err(format!("No task found with ID \"{}\".", id).into());
        }
    }

    storage::save_all(&tasks, "tasks")
}

pub fn task_todo() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = storage::load_task()?;
    let filtered_tasks: Vec<Task> = tasks.into_iter().filter(|task| !task.done).collect();

    if filtered_tasks.is_empty() {
        println!("No pending tasks")
    }
    print_tasks(&filtered_tasks);

    Ok(())
}

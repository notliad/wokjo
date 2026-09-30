use crate::entry::Entry;
use crate::storage;
use crate::task::Task;
use chrono::{Datelike, Duration, Local, NaiveDate};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

pub fn add(
    message: &str,
    tags: Vec<String>,
    date: Option<NaiveDate>,
) -> Result<(), Box<dyn std::error::Error>> {
    let entry = Entry::new(message, tags, date)?;
    let entry_date = entry.timestamp.date_naive();
    storage::save(&entry, "entries")?;

    println!("Entry added!");

    show_entries_for_date(entry_date)?;

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

fn entry_matches_filters(
    entry: &Entry,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    tag: Option<&str>,
) -> bool {
    let date = entry.timestamp.date_naive();

    from.is_none_or(|from| date >= from)
        && to.is_none_or(|to| date <= to)
        && tag.is_none_or(|tag| {
            entry
                .tags
                .iter()
                .any(|entry_tag| entry_tag.eq_ignore_ascii_case(tag))
        })
}

fn retain_latest(entries: &mut Vec<Entry>, limit: Option<usize>) {
    if let Some(limit) = limit {
        entries.drain(..entries.len().saturating_sub(limit));
    }
}

fn validate_date_range(from: Option<NaiveDate>, to: Option<NaiveDate>) -> Result<(), &'static str> {
    if from.zip(to).is_some_and(|(from, to)| from > to) {
        return Err("--from must be before or equal to --to");
    }

    Ok(())
}

pub fn list(
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    tag: Option<&str>,
    limit: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_date_range(from, to)?;

    let mut entries = storage::load()?;
    entries.retain(|entry| entry_matches_filters(entry, from, to, tag));
    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    retain_latest(&mut entries, limit);

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

fn tag_counts(entries: &[Entry]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();

    for entry in entries {
        let tags: BTreeSet<String> = entry
            .tags
            .iter()
            .map(|tag| tag.trim().to_lowercase())
            .filter(|tag| !tag.is_empty())
            .collect();

        for tag in tags {
            *counts.entry(tag).or_default() += 1;
        }
    }

    counts
}

pub fn tags() -> Result<(), Box<dyn std::error::Error>> {
    let entries = storage::load()?;
    let counts = tag_counts(&entries);

    if counts.is_empty() {
        println!("No tags found.");
        return Ok(());
    }

    for (tag, count) in counts {
        println!("[{}] {}", tag, count);
    }

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
    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

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
        println!(
            "{} {} - {}",
            entry.id,
            entry.formatted_time(),
            entry.message
        )
    }

    if current_day.is_none() {
        println!("No activities logged this week.");
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
            "{} {} - {} {}",
            entry.id,
            entry.formatted_time(),
            entry.formatted_tags(),
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

pub fn search(query: Option<&str>, tag: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = storage::load()?;
    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let lower_query = query.map(|query| query.to_lowercase());

    let filtered_entries: Vec<Entry> = entries
        .into_iter()
        .filter(|entry| {
            let matches_query = match &lower_query {
                Some(query) => entry.message.to_lowercase().contains(query),
                None => true,
            };

            let matches_tag = match tag {
                Some(tag) => entry
                    .tags
                    .iter()
                    .any(|entry_tag| entry_tag.eq_ignore_ascii_case(tag)),
                None => true,
            };

            matches_query && matches_tag
        })
        .collect();

    if filtered_entries.is_empty() {
        println!("No activities found.");
        return Ok(());
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

pub fn edit(
    id: &str,
    message: Option<&str>,
    tags: Vec<String>,
    date: Option<NaiveDate>,
) -> Result<(), Box<dyn std::error::Error>> {
    if message.is_none() && tags.is_empty() && date.is_none() {
        return Err("Provide a message, --date, or --tag".into());
    }

    let mut entries = storage::load()?;
    let entry = entries
        .iter_mut()
        .find(|entry| entry.id == id)
        .ok_or_else(|| format!("No entry found with ID \"{}\".", id))?;

    if let Some(message) = message {
        entry.set_message(message)?;
    }
    if !tags.is_empty() {
        entry.tags = tags;
    }
    if let Some(date) = date {
        entry.set_date(date)?;
    }

    storage::save_all(&entries, "entries")?;
    println!("Log edited!");

    Ok(())
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

fn entries_to_markdown(entries: &[Entry]) -> String {
    let mut markdown = String::from("# Work Log\n");
    let mut current_day: Option<NaiveDate> = None;

    for entry in entries {
        let entry_day = entry.timestamp.date_naive();
        if current_day != Some(entry_day) {
            writeln!(markdown, "\n## {}", entry_day.format("%d/%m/%Y")).unwrap();
            current_day = Some(entry_day);
        }
        writeln!(
            markdown,
            "- `{}`  {}",
            entry.formatted_time(),
            entry.message
        )
        .unwrap();
    }

    markdown
}

fn tasks_to_markdown(tasks: &[Task]) -> String {
    let mut markdown = String::from("# Work Tasks\n\n");

    for task in tasks {
        let status = if task.done { "[x]" } else { "[ ]" };
        writeln!(markdown, "- {} {}", status, task.message).unwrap();
    }

    markdown
}

pub fn export_entries(
    output: Option<&str>,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    tag: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_date_range(from, to)?;

    let mut entries = storage::load()?;
    entries.retain(|entry| entry_matches_filters(entry, from, to, tag));
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    let output = output.unwrap_or("wokjo-entries.md");

    let entries_markdowned = entries_to_markdown(&entries);

    std::fs::write(output, entries_markdowned)?;
    println!("Exported to {}", output);

    Ok(())
}

pub fn export_tasks(output: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = storage::load_task()?;
    let output = output.unwrap_or("wokjo-tasks.md");

    let tasks_markdowned = tasks_to_markdown(&tasks);

    std::fs::write(output, tasks_markdowned)?;
    println!("Exported to {}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_entry_filters_inclusively() {
        let date = NaiveDate::from_ymd_opt(2026, 9, 30).unwrap();
        let entry = Entry::new("Filtered log", vec!["Rust".to_string()], Some(date)).unwrap();

        assert!(entry_matches_filters(
            &entry,
            Some(date),
            Some(date),
            Some("rust")
        ));
        assert!(!entry_matches_filters(
            &entry,
            Some(date + Duration::days(1)),
            None,
            None
        ));
        assert!(!entry_matches_filters(&entry, None, None, Some("cli")));
    }

    #[test]
    fn should_retain_latest_entries_in_chronological_order() {
        let mut entries: Vec<Entry> = (28..=30)
            .map(|day| {
                Entry::new(
                    "Limited log",
                    Vec::new(),
                    Some(NaiveDate::from_ymd_opt(2026, 9, day).unwrap()),
                )
                .unwrap()
            })
            .collect();

        retain_latest(&mut entries, Some(2));

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].timestamp.date_naive().day(), 29);
        assert_eq!(entries[1].timestamp.date_naive().day(), 30);
    }

    #[test]
    fn should_count_each_tag_once_per_entry() {
        let entries = vec![
            Entry::new(
                "First log",
                vec!["Rust".to_string(), "rust".to_string()],
                None,
            )
            .unwrap(),
            Entry::new(
                "Second log",
                vec!["rust".to_string(), "cli".to_string()],
                None,
            )
            .unwrap(),
        ];

        assert_eq!(
            tag_counts(&entries),
            BTreeMap::from([("cli".to_string(), 1), ("rust".to_string(), 2)])
        );
    }
}

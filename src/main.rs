mod commands;
mod entry;
mod storage;
mod task;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wokjo")]
#[command(about = "Log your work activities", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Log a new work activity
    Add {
        /// Description of what you worked on
        message: String,
    },
    /// Show today's activities
    Today,
    /// Show yesterday's activities
    Yesterday,
    /// Show activities from that day
    Day {
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
    },
    /// List all logged activities
    List,
    /// Show activities from the current week
    Week,
    /// Search for a activity
    Search { query: String },
    /// Delete an entry
    Delete { id: String },
    /// Edit an entry
    Edit { id: String, message: String },
    /// Run wokjo task -h for more details
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    /// Run wokjo export -h for more details
    Export {
        #[command(subcommand)]
        command: ExportCommands,
    },
}

#[derive(Subcommand)]
enum TaskCommands {
    /// Add a task
    Add { message: String },
    /// List all tasks
    List,
    /// Check/uncheck a task
    Check { id: String },
    /// Edit a task
    Edit { id: String, message: String },
    /// Delete a task
    Delete { id: String },
    /// List all pending tasks
    Todo,
}

#[derive(Subcommand)]
enum ExportCommands {
    /// Export your activities to a md file
    Entries {
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Export your tasks to a md file
    Tasks {
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn parse_date(value: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(value, "%d/%m/%Y").map_err(|_| "Use DD/MM/YYYY format".to_string())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add { message } => commands::add(&message),
        Commands::Today => commands::today(),
        Commands::Yesterday => commands::yesterday(),
        Commands::Day { date } => commands::day(date),
        Commands::List => commands::list(),
        Commands::Week => commands::week(),
        Commands::Search { query } => commands::search(&query),
        Commands::Delete { id } => commands::delete(&id),
        Commands::Edit { id, message } => commands::edit(&id, &message),
        Commands::Task { command } => match command {
            TaskCommands::Add { message } => commands::task_add(&message),
            TaskCommands::List => commands::task_list(),
            TaskCommands::Check { id } => commands::task_check(&id),
            TaskCommands::Edit { id, message } => commands::task_edit(&id, &message),
            TaskCommands::Delete { id } => commands::task_delete(&id),
            TaskCommands::Todo => commands::task_todo(),
        },
        Commands::Export { command } => match command {
            ExportCommands::Entries { output } => commands::export_entries(output.as_deref()),
            ExportCommands::Tasks { output } => commands::export_tasks(output.as_deref()),
        },
    };

    if let Err(error) = result {
        println!("Error: {}", error);
    }
}

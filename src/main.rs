mod commands;
mod entry;
mod storage;
mod task;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use std::num::NonZeroUsize;

#[derive(Parser)]
#[command(name = "wokjo")]
#[command(
    about = "Log your work activities",
    version,
    after_help = "Run 'wokjo <command> --help' for command options."
)]
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

        /// Date of the activity in DD/MM/YYYY format
        #[arg(long, value_parser = parse_date)]
        date: Option<NaiveDate>,

        #[arg(short, long)]
        tag: Vec<String>,
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
    List {
        /// First date to include in DD/MM/YYYY format
        #[arg(long, value_parser = parse_date)]
        from: Option<NaiveDate>,

        /// Last date to include in DD/MM/YYYY format
        #[arg(long, value_parser = parse_date)]
        to: Option<NaiveDate>,

        #[arg(short, long)]
        tag: Option<String>,

        /// Number of most recent entries to show
        #[arg(long)]
        limit: Option<NonZeroUsize>,
    },
    /// Show activities from the current week
    Week,
    /// List tags and their entry counts
    Tags,
    /// Search for a activity
    Search {
        query: Option<String>,

        #[arg(short, long)]
        tag: Option<String>,
    },
    /// Delete an entry
    Delete { id: String },
    /// Edit an entry
    Edit {
        id: String,
        message: Option<String>,

        /// New date of the activity in DD/MM/YYYY format
        #[arg(long, value_parser = parse_date)]
        date: Option<NaiveDate>,

        /// Replace the entry tags
        #[arg(short, long)]
        tag: Vec<String>,
    },
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

        /// First date to include in DD/MM/YYYY format
        #[arg(long, value_parser = parse_date)]
        from: Option<NaiveDate>,

        /// Last date to include in DD/MM/YYYY format
        #[arg(long, value_parser = parse_date)]
        to: Option<NaiveDate>,

        #[arg(short, long)]
        tag: Option<String>,
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
        Commands::Add { message, date, tag } => commands::add(&message, tag, date),
        Commands::Today => commands::today(),
        Commands::Yesterday => commands::yesterday(),
        Commands::Day { date } => commands::day(date),
        Commands::List {
            from,
            to,
            tag,
            limit,
        } => commands::list(from, to, tag.as_deref(), limit.map(NonZeroUsize::get)),
        Commands::Week => commands::week(),
        Commands::Tags => commands::tags(),
        Commands::Search { query, tag } => commands::search(query.as_deref(), tag.as_deref()),
        Commands::Delete { id } => commands::delete(&id),
        Commands::Edit {
            id,
            message,
            date,
            tag,
        } => commands::edit(&id, message.as_deref(), tag, date),
        Commands::Task { command } => match command {
            TaskCommands::Add { message } => commands::task_add(&message),
            TaskCommands::List => commands::task_list(),
            TaskCommands::Check { id } => commands::task_check(&id),
            TaskCommands::Edit { id, message } => commands::task_edit(&id, &message),
            TaskCommands::Delete { id } => commands::task_delete(&id),
            TaskCommands::Todo => commands::task_todo(),
        },
        Commands::Export { command } => match command {
            ExportCommands::Entries {
                output,
                from,
                to,
                tag,
            } => commands::export_entries(output.as_deref(), from, to, tag.as_deref()),
            ExportCommands::Tasks { output } => commands::export_tasks(output.as_deref()),
        },
    };

    if let Err(error) = result {
        println!("Error: {}", error);
    }
}

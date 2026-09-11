mod commands;
mod entry;
mod storage;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wokjo")]
#[command(about = "Log your work activities")]
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
    // Search for a activity
    Search {
        query: String,
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
    };

    if let Err(error) = result {
        println!("Error: {}", error);
    }
}

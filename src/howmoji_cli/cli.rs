use clap::Parser;
use inquire::{validator::Validation, Select, Text};
use log::{debug, error};
use std::process::Command;

use crate::howmoji_config::config::Config;
use crate::howmoji_config::database::Database;
use crate::howmoji_config::howmoji::Howmoji;

#[derive(Parser, Debug)]
#[command(
    name = "howmoji",
    author,
    version,
    about = "Howmoji picker for git commits",
    long_about = None
)]
pub struct Arguments {
    #[arg(short, long)]
    pub command: bool,
    #[arg(short, long)]
    pub interactive: bool,
    #[arg(short, long)]
    pub emoji: Option<String>,
    #[arg(short, long)]
    pub title: Option<String>,
    #[arg(long)]
    pub message: Option<String>,
}

pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    debug!("Starting howmoji CLI");

    let args = Arguments::parse();

    let data_dir = match dirs::data_dir() {
        Some(mut path) => {
            path.push("howmoji");
            std::fs::create_dir_all(&path).unwrap_or_else(|_| {
                error!("Failed to create data directory");
                std::process::exit(1);
            });
            path
        }
        None => {
            error!("Could not determine data directory");
            std::process::exit(1);
        }
    };

    let db_path = data_dir.join("howmoji.db");
    let config = Config::new(db_path.to_string_lossy().into());
    debug!("Config loaded with JSON path: {}", config.json_path);

    let db = Database::new(&config);
    if !db_path.exists() || db.requires_update() {
        debug!("Database needs initialization");
        if let Err(e) = db.initialize() {
            error!("Failed to initialize database: {}", e);
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
        debug!("Database initialized successfully");
    } else {
        debug!("Using existing database");
    }

    // If emoji and title are provided, create commit directly
    if let (Some(emoji_arg), Some(title_arg)) = (&args.emoji, &args.title) {
        create_commit_with_args(emoji_arg, title_arg, args.message.as_deref());
        return;
    }

    if args.interactive || (args.emoji.is_none() && args.title.is_none()) {
        match db.get_howmojis() {
            Ok(howmojis) => {
                if howmojis.is_empty() {
                    error!("No emojis found in database");
                    eprintln!("No emojis found in database");
                    return;
                }

                interactive_mode(howmojis);
            }
            Err(e) => {
                error!("Failed to retrieve emojis: {}", e);
                eprintln!("Failed to retrieve emojis: {}", e);
            }
        }
    } else {
        eprintln!("Missing required arguments. Use --help for usage information.");
    }
}

// Function to handle interactive mode
// This function prompts the user to select an emoji and enter a commit message
// and then creates a git commit with the selected emoji and message.
fn interactive_mode(howmojis: Vec<Howmoji>) {
    debug!("Starting interactive mode with {} emojis", howmojis.len());

    // Format options for display
    let options: Vec<String> = howmojis
        .iter()
        .map(|h| format!("{} {}", h.emoji, h.description))
        .collect();

    // Prompt user to select emoji
    let answer = Select::new("Choose your howmoji:", options)
        .with_page_size(10)
        .prompt();

    let selected_emoji = match answer {
        Ok(selection) => {
            // Extract just the emoji part
            let emoji_char = selection.chars().next().unwrap_or(' ');
            emoji_char.to_string()
        }
        Err(err) => {
            // If user cancels, we don't need to do anything
            match err {
                inquire::InquireError::OperationCanceled
                | inquire::InquireError::OperationInterrupted => {
                    return;
                }
                _ => {
                    error!("Error getting emoji input: {}", err);
                    eprintln!("Something unexpected occurred: {}", err);
                    return;
                }
            }
        }
    };

    // Title validator
    let title_validator = |input: &str| {
        if input.chars().count() > 140 {
            Ok(Validation::Invalid(
                "You're only allowed 140 characters.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let title = loop {
        let title_input = Text::new("Enter a title for your commit message:")
            .with_validator(title_validator)
            .prompt();
        match title_input {
            Ok(input) => {
                if input.trim().is_empty() {
                    eprintln!("Title cannot be empty. Please enter a valid title.");
                    continue;
                }
                break input;
            }
            Err(err) => match err {
                inquire::InquireError::OperationCanceled
                | inquire::InquireError::OperationInterrupted => {
                    return;
                }
                _ => {
                    error!("Error getting description input: {}", err);
                    eprintln!("Something unexpected occurred: {}", err);
                }
            },
        }
    };

    // Description validator
    let description_validator = |input: &str| {
        if input.chars().count() > 500 {
            Ok(Validation::Invalid("Only allowed 500 characters".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    // Prompt for optional commit description
    let description = Text::new("Enter a commit message (optional)")
        .with_validator(description_validator)
        .prompt();

    match description {
        Ok(description) => {
            create_commit_with_args(
                &selected_emoji,
                &title,
                if description.is_empty() {
                    None
                } else {
                    Some(&description)
                },
            );
        }
        Err(err) => {
            // If user cancels, we don't need to do anything
            match err {
                inquire::InquireError::OperationCanceled
                | inquire::InquireError::OperationInterrupted => {
                    return;
                }
                _ => {
                    error!("Error getting description input: {}", err);
                    eprintln!("Something unexpected occurred: {}", err);
                }
            }
        }
    }
}

fn create_commit_with_args(emoji: &str, title: &str, description: Option<&str>) {
    let commit_cmd = if let Some(desc) = description {
        debug!(
            "Creating commit with emoji: {}, title: {}, and description",
            emoji, title
        );
        format!(
            "git commit -m \"{}\" -m \"{}\"",
            format!("{} {}", emoji, title),
            desc
        )
    } else {
        debug!("Creating commit with emoji: {} and title: {}", emoji, title);
        format!("git commit -m \"{}\"", format!("{} {}", emoji, title))
    };

    debug!("Executing command: {}", commit_cmd);

    match Command::new("sh").arg("-c").arg(&commit_cmd).status() {
        Ok(status) => {
            if status.success() {
                debug!("Successfully executed git commit");
            } else {
                error!("Failed to execute git commit command");
                eprintln!("Failed to execute git commit command");
            }
        }
        Err(err) => {
            error!("Error executing git command: {}", err);
            eprintln!("Failed to execute command: {}", err);
        }
    }
}

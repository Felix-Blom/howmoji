use clap::Parser;
use inquire::{
    validator::{StringValidator, Validation},
    Select, Text,
};
use serde_json;
use std::{f32::consts::PI, fs::File};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    emoji: String,

    #[arg(short, long)]
    title: String,

    #[arg(short, long)]
    content: Option<String>,
}

struct HowmojiOption {
    emoji: String,
    description: String,
}

fn main() {
    let file = File::open("config/howmoji.json").expect("Unable to read file");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("File should be proper json");

    let emoji_list = json["gitmojis"].as_array().expect("Expected an array");
    let options: Vec<String> = emoji_list
        .iter()
        .map(|e| {
            format!(
                "{} {}",
                e["emoji"].as_str().unwrap_or(""),
                e["description"].as_str().unwrap_or(""),
            )
        })
        .collect();

    let answer = Select::new("Choose your howmoji: ", options)
        .with_page_size(10)
        .prompt();

    let emoji = match answer {
        Ok(selection) => selection,
        Err(err) => {
            println!("Something unexpected happened: {}", err);
            return;
        }
    };

    // Create commit title
    let title_validator = |input: &str| {
        if input.chars().count() > 140 {
            Ok(Validation::Invalid(
                "You're only allowed 140 characters.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let mut title = String::new();
    loop {
        let title_input = Text::new("Enter a title for your commit message:")
            .with_validator(title_validator)
            .prompt();

        match title_input {
            Ok(input) => {
                if input.trim().is_empty() {
                    println!("Title cannot be empty. Please enter a valid title.");
                    continue;
                }
                title = input;
                break;
            }
            Err(err) => {
                println!("Something strange happened {}", err);
                return;
            }
        }
    }

    let description_validator = |input: &str| {
        if input.chars().count() > 500 {
            Ok(Validation::Invalid("Only allowed 500 characters".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    // Ask commit message
    let description = Text::new("Enter a commit message (optional)")
        .with_validator(description_validator)
        .prompt();

    let final_commit_message = match description {
        Ok(description) => {
            // Extract just the emoji character from the selection string
            let emoji_char = emoji.chars().next().unwrap_or(' ');

            if description.is_empty() {
                format!("git commit -m \"{}\"", format!("{} {}", emoji_char, title))
            } else {
                format!(
                    "git commit -m \"{}\" -m \"{}\"",
                    format!("{} {}", emoji_char, title),
                    description
                )
            }
        }
        Err(err) => {
            println!("Something unexpected occured {}!", err);
            return;
        }
    };

    match std::process::Command::new("sh")
        .arg("-c")
        .arg(&final_commit_message)
        .status()
    {
        Ok(status) => {
            if !status.success() {
                println!("Failed to execute git commit command");
            }
        }
        Err(err) => {
            println!("Failed to execute command: {}", err);
        }
    }
}

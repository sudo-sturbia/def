mod command;
mod errors;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use colored::*;
use command::Config;
use ddir::Describer;
use errors::Handle;

static CONFIG_FILE: &str = ".config/ddir/config.json";
static CONFIG_DIR: &str = ".config/ddir";

const JSON_PRETTY: bool = true; // Use pretty JSON

fn main() {
    match command::parse(&env::args().collect::<Vec<String>>()) {
        Ok(config) => {
            if config.help {
                usage()
            }

            match &config.description {
                Some(description) => add_description(&config, &description),
                None => print_description(&config),
            }
        }
        Err(e) => eprintln!("{}: {}", "Err".red(), e.message),
    }
}

/// usage prints a help message to stderr and exits with exit code 1.
fn usage() {
    eprintln!("");
    process::exit(1);
}

/// add_description creates a describer, either from CONFIG_FILE if it exists,
/// or empty otherwise. Maps the given description to config.path, and (re)writes
/// the describer to CONFIG_FILE.
fn add_description(config: &Config, description: &str) {
    let mut describer = if Path::new(&full_path(CONFIG_FILE)).exists() {
        get_describer()
    } else {
        fs::create_dir_all(&full_path(CONFIG_DIR)).extract_or_exit("failed to create config");
        Describer::new()
    };

    if config.add_description {
        describer.add_description(config.path.as_ref().unwrap(), description);
    } else if config.add_pattern {
        describer.add_pattern(config.path.as_ref().unwrap(), description);
    }

    fs::write(
        &full_path(CONFIG_FILE),
        describer
            .to_json(JSON_PRETTY)
            .extract_or_exit("failed to create config"),
    )
    .extract_or_exit("failed to write config to file")
}

/// print_description creates a describer using CONFIG_FILE, and prints
/// a description of the path specified in config.path. If no description
/// exists, an error message is printed.
fn print_description(config: &Config) {
    let describer = get_describer();
    let path = config.path.as_ref().unwrap(); // Guaranteed to have a value.
    println!(
        "{}",
        match describer.describe(&path) {
            Some(description) => format!("{}: {}", path.green(), description),
            None => format!("{}: {}", "Err".red(), "no available description"),
        }
    )
}

/// get_describer loads JSON from CONFIG_FILE, creates a describer and
/// returns it. Exits on error.
fn get_describer() -> ddir::Describer {
    Describer::new_from_json(
        &fs::read_to_string(&full_path(CONFIG_FILE)).extract_or_exit("failed to read config"),
    )
    .extract_or_exit("invalid JSON config")
}

/// full_path takes a path relative to $HOME and returns an absolute path.
/// Exits on failure.
fn full_path(relative: &str) -> String {
    format!(
        "{}/{}",
        env::var("HOME").extract_or_exit("failed to get $HOME"),
        relative
    )
}

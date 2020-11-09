mod command;
mod errors;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use colored::*;
use command::InvokedTo;
use def::Describer;
use errors::Handle;

const JSON_PRETTY: bool = true; // Use pretty JSON

fn main() {
    match command::parse(&env::args().collect::<Vec<String>>()) {
        InvokedTo::ShortHelp => help(),
        InvokedTo::Help => usage(),
        InvokedTo::DescribePath(p) => print_description(&p),
        InvokedTo::AddDescription(p, d) => add_description(&p, &d, false),
        InvokedTo::AddPattern(p, d) => add_description(&p, &d, true),
        InvokedTo::Unknown => eprintln!("{}: {}", "Err".red(), "invalid argument list"),
    }
}

/// help prints a short help message to stderr and exits with code 1.
fn help() {
    eprintln!(
        "{}{}{}",
        "Usage\n",
        "  def [ <path> | add <path> <description> | pattern <path> <description> ]\n",
        "Try \"def help\" for more details.",
    );
    process::exit(1);
}

/// usage prints a help message to stderr and exits with exit code 1.
fn usage() {
    eprintln!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "def keeps track of file and directory descriptions for you.\n",
        "\n",
        "Usage\n",
        "\n",
        "  def <path>                        Print description of file/dir at path.\n",
        "  def add <path> <description>      Add a description for file/dir at path.\n",
        "  def pattern <path> <description>  Add a pattern to describe children of dir. A\n",
        "                                    wildcard in the pattern will be replaced with\n",
        "                                    the child's name.\n",
        "\n",
        "Descriptions\n",
        "\n",
        "  To describe a file or directory use add sub-command which simply maps a description\n",
        "  to an absolute path.\n",
        "\n",
        "  The pattern sub-command is used to describe all children of a directory using a\n",
        "  common trait. When pattern is used, a description is mapped to a dir, but is used\n",
        "  only to describe its children. If a wildcard \"*\" exists in the pattern, it will\n",
        "  be replaced by the child's name.\n",
        "\n",
        "  For example:\n",
        "\n",
        "  $ def pattern dir \"* is a child of dir\"\n",
        "  $ def dir/temp\n",
        "  /path/to/dir/temp: temp is a child of dir\n",
        "\n",
        "Descriptions and patterns are kept in ~/.config/def/config.json which maps each\n",
        "description to an absolute path and can be added to or adjusted manually.",
    );
    process::exit(1);
}

/// add_description creates a describer, either from config_file if it exists,
/// or empty otherwise. Maps the given description to path, and (re)writes the
/// describer to config_file.
fn add_description(path: &str, description: &str, pattern: bool) {
    let mut describer = if Path::new(&config_file()).exists() {
        get_describer()
    } else {
        fs::create_dir_all(&config_dir()).extract_or_exit("failed to create config");
        Describer::new()
    };

    if pattern {
        describer.add_pattern(&absolute_path(path), description);
    } else {
        describer.add_description(&absolute_path(path), description);
    }

    fs::write(
        &config_file(),
        describer
            .to_json(JSON_PRETTY)
            .extract_or_exit("failed to create config"),
    )
    .extract_or_exit("failed to write config to file")
}

/// print_description creates a describer using config_file, and prints
/// a description of the specified path. If no description exists, an error
/// message is printed.
fn print_description(path: &str) {
    let describer = get_describer();
    let path = absolute_path(path);
    println!(
        "{}",
        match describer.describe(&path) {
            Some(description) => format!("{}: {}", path.green(), description),
            None => format!("{}: {}", "Err".red(), "no available description"),
        }
    )
}

/// get_describer loads JSON from config_file, creates a describer and
/// returns it. Exits on error.
fn get_describer() -> def::Describer {
    Describer::new_from_json(
        &fs::read_to_string(&config_file()).extract_or_exit("failed to read config"),
    )
    .extract_or_exit("invalid JSON config")
}

/// config_file returns path to configuration file.
fn config_file() -> String {
    format!(
        "{}/.config/def/config.json",
        env::var("HOME").extract_or_exit("failed to get $HOME"),
    )
}

/// config_dir returns path to directory containing configuration file.
fn config_dir() -> String {
    format!(
        "{}/.config/def",
        env::var("HOME").extract_or_exit("failed to get $HOME"),
    )
}

/// absolute_path takes a path and returns its absolute representation.
/// Exits on failure (if path doesn't exist).
fn absolute_path(path: &str) -> String {
    match fs::canonicalize(path)
        .extract_or_exit("failed to get absolute path")
        .to_str()
    {
        Some(p) => p.to_string(),
        None => {
            eprintln!("{}: {}", "Err".red(), "path contains invalid chars");
            process::exit(1);
        }
    }
}

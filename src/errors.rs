use std::fmt::Display;
use std::process;

use colored::*;

/// Handle defines a method to extract a value from a Result and exit on error. It's
/// created to avoid code repetition.
pub trait Handle<T> {
    /// extract_or_exit attempts to extract a value from a Result. If the value
    /// doesn't exist, an error message is printed and the program exits with error
    /// code 1. This is basically a prettier version of Result.expect.
    fn extract_or_exit(self, message: &str) -> T;
}

impl<T, E: Display> Handle<T> for Result<T, E> {
    fn extract_or_exit(self, message: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}: {}: {}", "Err".red(), message, e);
                process::exit(1);
            }
        }
    }
}

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::error;

/// Directory seperator. Used to split a string.
const SEPERATOR: char = '/';

/// A place holder in patterns. Replaced with working directory's name.
const DIRECTORY_PLACEHOLDER: char = '*';

/// Describer holds descriptions of directories.
///
/// # Types of Descriptions
///
/// - Specific directory description: A string mapped to a directory's path
/// describing the directory. When describe is called this will be retrieved
/// as is.
/// - Pattern description: A string mapped to a directory's path describing
/// the directory's children. In patterns, "*" is interpreted as a place holder
/// for working directory's name, and are replaced by the name when retreived.
///
/// # Examples
///
/// ```
/// ```
#[derive(Deserialize, Debug)]
pub struct Describer {
    descriptions: HashMap<String, String>,
    patterns: HashMap<String, String>,
}

impl Describer {
    /// Create and return a new describer using given HashMaps.
    ///
    /// # Arguments
    ///
    /// * `d` - A map of directory descriptions.
    /// * `p` - A map of patterns.
    pub fn new(d: HashMap<String, String>, p: HashMap<String, String>) -> Describer {
        Describer {
            descriptions: d,
            patterns: p,
        }
    }

    /// Create and return a new describer using the given JSON value.
    ///
    /// # Arguments
    ///
    /// * `json` - A string representing a JSON value that can be deserialized
    /// into a Describer. An error is returned if the JSON string can't be
    /// deserialized.
    pub fn new_from_json(json: &str) -> Result<Describer, error::Error> {
        serde_json::from_str::<Describer>(json)
    }

    /// Return a description of the given path or None if no description
    /// exists. The descriptions map is checked for a description first,
    /// if none is found, then the patterns map is checked.
    pub fn describe(&self, directory: &str) -> Option<String> {
        match self.descriptions.get(directory) {
            Some(d) => Some(d.clone()),
            None => self.describe_using_pattern(directory),
        }
    }

    /// Check patterns map for a description. If one exists, return it with
    /// all place holders replaced, otherwise return None.
    fn describe_using_pattern(&self, directory: &str) -> Option<String> {
        let v: Vec<&str> = directory.rsplitn(2, SEPERATOR).collect();
        if v.len() != 2 {
            None
        } else {
            match self.patterns.get(v[1]) {
                Some(p) => Some(p.replace(DIRECTORY_PLACEHOLDER, v[0])),
                None => None,
            }
        }
    }
}

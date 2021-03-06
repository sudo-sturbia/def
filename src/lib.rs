//! def is a library backing def command line tool. It mainly provides `Describer`
//! structwhich is used to map string descriptions to paths and retrieve them when
//! needed.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Directory seperator. Used to split a string.
const SEPERATOR: char = '/';

/// A place holder in patterns. Replaced with a name.
const NAME_PLACEHOLDER: char = '*';

/// Describer holds descriptions of files and directories.
///
/// # Types of Descriptions
///
/// - Specific description: A string mapped to a path describing a file or directory.
/// When describe is called this will be retrieved as is.
/// - Pattern description: A string mapped to a directory's path describing a child
/// of the directory. When description of a child is wanted, the pattern is retrieved.
/// In patterns, a wildcard is interpreted as a place holder for child's name, and are
/// replaced by the name when retreived.
///
/// If a string can be described using both a pattern and a specific description,
/// the specific description will be favoured.
///
/// # Examples
///
/// ```
/// // Create a mutable describer.
/// let mut describer = def::Describer::new();
///
/// // Map a description to a given path.
/// describer.add_description("path/to/directory", "This is an empty directory.");
///
/// // Map a pattern to a given path. The pattern applies to the path's
/// // children. "*" works as a placeholder and will be replaced by the
/// // child's name.
/// describer.add_pattern("parent/directory", "* is a child of parent/directory.");
///
/// // The description is retrieved as is.
/// assert_eq!(
///     describer.describe("path/to/directory"),
///     Some("This is an empty directory.".to_string())
/// );
///
/// // "*" is replaced with "test".
/// assert_eq!(
///     describer.describe("parent/directory/test"),
///     Some("test is a child of parent/directory.".to_string())
/// );
///
/// // Despite having a pattern mapped to it, the pattern only applies to
/// // its children.
/// assert_eq!(describer.describe("parent/directory"), None);
/// ```
///
#[derive(Deserialize, Serialize, Debug)]
pub struct Describer {
    descriptions: HashMap<String, String>,
    patterns: HashMap<String, String>,
}

impl Describer {
    /// Create and return a new empty describer.
    pub fn new() -> Describer {
        Describer {
            descriptions: HashMap::new(),
            patterns: HashMap::new(),
        }
    }

    /// Create and return a new describer using given HashMaps.
    ///
    /// # Arguments
    ///
    /// * `d` - A map of descriptions.
    /// * `p` - A map of patterns.
    pub fn new_with(d: HashMap<String, String>, p: HashMap<String, String>) -> Describer {
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
    pub fn new_from_json(json: &str) -> Result<Describer, serde_json::Error> {
        serde_json::from_str::<Describer>(json)
    }

    /// Return a description of the given path or None if no description
    /// exists. The descriptions map is checked for a description first,
    /// if none is found, then the patterns map is checked.
    pub fn describe(&self, path: &str) -> Option<String> {
        match self.descriptions.get(path) {
            Some(d) => Some(d.clone()),
            None => self.describe_using_pattern(path),
        }
    }

    /// Check patterns map for a description. If one exists, return it with
    /// all place holders replaced, otherwise return None.
    fn describe_using_pattern(&self, path: &str) -> Option<String> {
        let parent: Vec<&str> = path.rsplitn(2, SEPERATOR).collect();
        if parent.len() != 2 {
            None
        } else {
            match self.patterns.get(parent[1]) {
                Some(p) => Some(p.replace(NAME_PLACEHOLDER, parent[0])),
                None => None,
            }
        }
    }

    /// Add a description to the descriptions map.
    pub fn add_description(&mut self, path: &str, desc: &str) {
        self.descriptions.insert(path.to_string(), desc.to_string());
    }

    /// Add a pattern to the patterns map.
    pub fn add_pattern(&mut self, path: &str, desc: &str) {
        self.patterns.insert(path.to_string(), desc.to_string());
    }

    /// Return a string JSON representation of this Describer. This is
    /// subsequently written to a file to be re-loaded on next run.
    ///
    /// # Arguments
    ///
    /// * `pretty` - If true, return a "pretty" JSON string.
    pub fn to_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
    }
}

impl Default for Describer {
    fn default() -> Describer {
        Describer::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_describe_test() {
        let mut descriptions: HashMap<String, String> = HashMap::new();
        let mut patterns: HashMap<String, String> = HashMap::new();
        for (path, desc, is_pattern) in [
            ("/path/to/dir", "This is /path/to/dir.", false),
            ("/another/dir", "This is /another/dir.", false),
            ("/yet/another/path", "This is /yet/another/path.", false),
            ("/path/to/dir", "* is in /path/to/dir.", true),
            ("/yet/another/path", "* is in /yet/another/path.", true),
            ("/obvious", "* is *", true),
            ("/yet/another", "* is in /yet/another/path.", true),
        ]
        .iter()
        {
            if *is_pattern {
                patterns.insert(path.to_string(), desc.to_string());
            } else {
                descriptions.insert(path.to_string(), desc.to_string());
            }
        }

        describe_tester(&Describer::new_with(descriptions, patterns));
    }

    #[test]
    fn new_from_json_describe_test() {
        match Describer::new_from_json(
            "
	    {
                \"descriptions\": {
                        \"/path/to/dir\": \"This is /path/to/dir.\",
                        \"/another/dir\": \"This is /another/dir.\",
                        \"/yet/another/path\": \"This is /yet/another/path.\"
		},
                \"patterns\": {
                        \"/path/to/dir\": \"* is in /path/to/dir.\",
                        \"/yet/another/path\": \"* is in /yet/another/path.\",
                        \"/obvious\": \"* is *\",
                        \"/yet/another\": \"* is in /yet/another/path.\"
                }
            }",
        ) {
            Ok(d) => describe_tester(&d),
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn add_test() {
        let mut d = Describer::new();
        d.add_description("path/to/directory", "This is an empty directory.");
        d.add_pattern("parent/directory", "* is a child of parent/directory.");
        assert_eq!(
            d.to_json(false).unwrap(),
            format!(
                "{}{}{}{}",
                "{\"descriptions\":",
                "{\"path/to/directory\":\"This is an empty directory.\"},",
                "\"patterns\":",
                "{\"parent/directory\":\"* is a child of parent/directory.\"}}"
            )
        );
    }

    fn describe_tester(describer: &Describer) {
        for (path, desc, is_none) in [
            ("/path/to/dir", "This is /path/to/dir.", false),
            ("/another/dir", "This is /another/dir.", false),
            ("/yet/another/path", "This is /yet/another/path.", false),
            ("/path/to/dir/1", "1 is in /path/to/dir.", false),
            ("/path/to/dir/things", "things is in /path/to/dir.", false),
            ("/yet/another/path/1", "1 is in /yet/another/path.", false),
            ("/yet/another/path/$", "$ is in /yet/another/path.", false),
            ("/obvious/obviously", "obviously is obviously", false),
            ("/doesn't/exist", "", true),
        ]
        .iter()
        {
            assert_eq!(
                describer.describe(path),
                if *is_none {
                    None
                } else {
                    Some(desc.to_string())
                }
            );
        }
    }
}

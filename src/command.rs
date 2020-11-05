// Available command line flags.
const ADD_FLAG: &str = "-add";
const PATTERN_FLAG: &str = "-pattern";
const HELP_FLAG: &str = "-help";

/// Result of command line parsing.
type Result = std::result::Result<Config, ConfigError>;

/// InvokedTo defines different things the `ddir` command can do, such as:
/// print a help message, describe current working directory, etc. Only
/// one of these things can be done at a time depending on how the command
/// is used. In addition to describing what the command can do, each enum
/// also contains the parameters needed to perform the operation, which should
/// be extracted from the command line argumenst.
///
/// For example: `ddir -add path describition` is parsed to
/// `InvokedTo::AddDescription("path", "description")`.
///
/// If new functionality is added to the command (such as a new flag), then
/// a new enum defining it should be added here.
#[derive(Debug, PartialEq)]
enum InvokedTo {
    Help,
    DescribeDirAtPath(String),
    AddDescription(String, String),
    AddPattern(String, String),
    Unknown,
}

/// Config collects the data needed for the ddir command and acts as a
/// parsing result.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub path: Option<String>,
    pub description: Option<String>,
    pub help: bool,
    pub add_description: bool,
    pub add_pattern: bool,
}

/// An error detected in command line arguments/flags.
#[derive(Debug, PartialEq)]
pub struct ConfigError {
    pub message: String,
}

/// parse parses command line arguments and returns a Config object containing
/// extracted data or a Config error if arguments don't match a defined pattern.
pub fn parse(args: &[String]) -> Result {
    match invocation_pattern(args) {
        InvokedTo::Help => Config::wrap(None, None, true, false, false),
        InvokedTo::DescribeDirAtPath(p) => Config::wrap(Some(p), None, false, false, false),
        InvokedTo::AddDescription(p, d) => Config::wrap(Some(p), Some(d), false, true, false),
        InvokedTo::AddPattern(p, d) => Config::wrap(Some(p), Some(d), false, false, true),
        InvokedTo::Unknown => ConfigError::wrap("invalid argument list"),
    }
}

/// invocation_pattern parses a list of command line arguments and returns
/// an enum describing what the command should achieve (print a help message,
/// print description of dir, add description, etc.), and a list of arguments
/// needed to do it.
fn invocation_pattern(args: &[String]) -> InvokedTo {
    match args.len() {
        1 => InvokedTo::Help,
        2 => match args[1].as_str() {
            HELP_FLAG => InvokedTo::Help,
            _ => InvokedTo::DescribeDirAtPath(args[1].clone()),
        },
        4 => match args[1].as_str() {
            ADD_FLAG => InvokedTo::AddDescription(args[2].clone(), args[3].clone()),
            PATTERN_FLAG => InvokedTo::AddPattern(args[2].clone(), args[3].clone()),
            _ => InvokedTo::Unknown,
        },
        _ => InvokedTo::Unknown,
    }
}

impl Config {
    fn wrap(path: Option<String>, desc: Option<String>, help: bool, d: bool, p: bool) -> Result {
        Ok(Config {
            path: match path {
                Some(s) => Some(s),
                None => None,
            },
            description: match desc {
                Some(s) => Some(s),
                None => None,
            },
            help,
            add_description: d,
            add_pattern: p,
        })
    }
}

impl ConfigError {
    fn wrap(message: &str) -> Result {
        Err(ConfigError {
            message: message.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        for (args, config) in [
            (
                vec!["ddir".to_string()],
                Config::wrap(None, None, true, false, false),
            ),
            (
                vec!["./renamed".to_string()],
                Config::wrap(None, None, true, false, false),
            ),
            (
                vec!["ddir".to_string(), "-help".to_string()],
                Config::wrap(None, None, true, false, false),
            ),
            (
                vec!["ddir".to_string(), "/path/to/dir".to_string()],
                Config::wrap(Some("/path/to/dir".to_string()), None, false, false, false),
            ),
            (
                vec![
                    "ddir".to_string(),
                    "-add".to_string(),
                    "/path".to_string(),
                    "description".to_string(),
                ],
                Config::wrap(
                    Some("/path".to_string()),
                    Some("description".to_string()),
                    false,
                    true,
                    false,
                ),
            ),
            (
                vec![
                    "ddir".to_string(),
                    "-pattern".to_string(),
                    "/path".to_string(),
                    "description".to_string(),
                ],
                Config::wrap(
                    Some("/path".to_string()),
                    Some("description".to_string()),
                    false,
                    false,
                    true,
                ),
            ),
            (vec![], ConfigError::wrap("invalid argument list")),
            (
                vec![
                    "ddir".to_string(),
                    "path".to_string(),
                    "another".to_string(),
                ],
                ConfigError::wrap("invalid argument list"),
            ),
            (
                vec![
                    "ddir".to_string(),
                    "-add".to_string(),
                    "path".to_string(),
                    "another".to_string(),
                    "another".to_string(),
                ],
                ConfigError::wrap("invalid argument list"),
            ),
            (
                vec![
                    "ddir".to_string(),
                    "-unkown".to_string(),
                    "path".to_string(),
                    "description".to_string(),
                ],
                ConfigError::wrap("invalid argument list"),
            ),
        ]
        .iter()
        {
            assert_eq!(parse(args), *config)
        }
    }

    #[test]
    fn invocation_pattern_test() {
        for (args, res) in [
            (vec!["ddir".to_string()], InvokedTo::Help),
            (vec!["./renamed".to_string()], InvokedTo::Help),
            (
                vec!["ddir".to_string(), "-help".to_string()],
                InvokedTo::Help,
            ),
            (
                vec!["ddir".to_string(), "/path/to/dir".to_string()],
                InvokedTo::DescribeDirAtPath("/path/to/dir".to_string()),
            ),
            (
                vec![
                    "ddir".to_string(),
                    "-add".to_string(),
                    "/path".to_string(),
                    "description".to_string(),
                ],
                InvokedTo::AddDescription("/path".to_string(), "description".to_string()),
            ),
            (
                vec![
                    "ddir".to_string(),
                    "-pattern".to_string(),
                    "/path".to_string(),
                    "description".to_string(),
                ],
                InvokedTo::AddPattern("/path".to_string(), "description".to_string()),
            ),
            (vec![], InvokedTo::Unknown),
            (
                vec![
                    "ddir".to_string(),
                    "path".to_string(),
                    "another".to_string(),
                ],
                InvokedTo::Unknown,
            ),
            (
                vec![
                    "ddir".to_string(),
                    "-add".to_string(),
                    "path".to_string(),
                    "another".to_string(),
                    "another".to_string(),
                ],
                InvokedTo::Unknown,
            ),
            (
                vec![
                    "ddir".to_string(),
                    "-unkown".to_string(),
                    "path".to_string(),
                    "description".to_string(),
                ],
                InvokedTo::Unknown,
            ),
        ]
        .iter()
        {
            assert_eq!(invocation_pattern(args), *res)
        }
    }
}
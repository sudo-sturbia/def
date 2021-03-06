// def's sub-commands.
const ADD_COMMAND: &str = "add";
const PATTERN_COMMAND: &str = "pattern";
const HELP_COMMAND: &str = "help";

/// InvokedTo defines different things the `def` command can do, such as:
/// print a help message, describe directory, add a description, etc. Only
/// one of these things can be done at a time depending on how the command
/// is used. In addition to describing what the command can do, each enum
/// also contains the parameters needed to perform the operation, which should
/// be extracted from the command line argumenst.
///
/// For example: `def add path describition` is parsed to
/// `InvokedTo::AddDescription("path", "description")`.
///
/// If new functionality is added to the command (such as a new flag), then
/// a new enum defining it should be added here.
#[derive(Debug, PartialEq)]
pub enum InvokedTo {
    ShortHelp,
    Help,
    DescribePath(String),
    AddDescription(String, String),
    AddPattern(String, String),
    Unknown,
}

/// parse parses a list of command line arguments and returns  an enum describing
/// what the command should achieve (print a help message, print description, add
/// description, etc.), and a list of arguments needed to do it.
pub fn parse(args: &[String]) -> InvokedTo {
    match args.len() {
        1 => InvokedTo::ShortHelp,
        2 => match args[1].as_str() {
            HELP_COMMAND => InvokedTo::Help,
            _ => InvokedTo::DescribePath(args[1].clone()),
        },
        4 => match args[1].as_str() {
            ADD_COMMAND => InvokedTo::AddDescription(args[2].clone(), args[3].clone()),
            PATTERN_COMMAND => InvokedTo::AddPattern(args[2].clone(), args[3].clone()),
            _ => InvokedTo::Unknown,
        },
        _ => InvokedTo::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        for (args, res) in [
            (vec!["def".to_string()], InvokedTo::ShortHelp),
            (vec!["./renamed".to_string()], InvokedTo::ShortHelp),
            (vec!["def".to_string(), "help".to_string()], InvokedTo::Help),
            (
                vec!["def".to_string(), "/path/to/dir".to_string()],
                InvokedTo::DescribePath("/path/to/dir".to_string()),
            ),
            (
                vec![
                    "def".to_string(),
                    "add".to_string(),
                    "/path".to_string(),
                    "description".to_string(),
                ],
                InvokedTo::AddDescription("/path".to_string(), "description".to_string()),
            ),
            (
                vec![
                    "def".to_string(),
                    "pattern".to_string(),
                    "/path".to_string(),
                    "description".to_string(),
                ],
                InvokedTo::AddPattern("/path".to_string(), "description".to_string()),
            ),
            (vec![], InvokedTo::Unknown),
            (
                vec!["def".to_string(), "path".to_string(), "another".to_string()],
                InvokedTo::Unknown,
            ),
            (
                vec![
                    "def".to_string(),
                    "add".to_string(),
                    "path".to_string(),
                    "another".to_string(),
                    "another".to_string(),
                ],
                InvokedTo::Unknown,
            ),
            (
                vec![
                    "def".to_string(),
                    "-unkown".to_string(),
                    "path".to_string(),
                    "description".to_string(),
                ],
                InvokedTo::Unknown,
            ),
        ]
        .iter()
        {
            assert_eq!(parse(args), *res)
        }
    }
}

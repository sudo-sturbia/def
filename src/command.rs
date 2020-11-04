// Available command line flags.
const ADD_FLAG: &str = "-add";
const PATTERN_FLAG: &str = "-pattern";
const HELP_FLAG: &str = "-help";

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
pub enum InvokedTo {
    Help,
    DescribeDirAtPath(String),
    AddDescription(String, String),
    AddPattern(String, String),
    Unknown,
}

/// parse parses a list of command line arguments and returns an enum
/// describing what the command should achieve (print a help message,
/// print description of dir, add description, etc.), and a list of
/// arguments needed to do it.
pub fn parse(args: &Vec<String>) -> InvokedTo {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
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
            assert_eq!(parse(args), *res)
        }
    }
}

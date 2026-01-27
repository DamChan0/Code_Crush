#[derive(Debug, PartialEq)]
pub enum Command {
    Search {
        pattern: String,
        path: String,
        case_insensitive: bool,
    },
    Quit,
    Help,
    Invalid(String),
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, String> {
        let input_arg = input.trim();

        if input_arg.is_empty() {
            return Ok(Self::Invalid("empty input".to_string()));
        }

        match input_arg {
            "quit" | "q" | "exit" => Ok(Self::Quit),
            "help" | "h" => Ok(Self::Help),
            _ => {
                let parts: Vec<&str> = input_arg.split_whitespace().collect();
                let pattern = parts[0].to_string();
                let mut case_insensitive = false;
                let mut path: Option<String> = None;

                for part in parts.iter().skip(1) {
                    match *part {
                        "-i" | "--ignore-case" => case_insensitive = true,
                        _ if part.starts_with('-') => {
                            return Ok(Self::Invalid(format!("unknown flag: {}", part)));
                        }
                        _ => {
                            if path.is_none() {
                                path = Some((*part).to_string());
                            } else {
                                return Ok(Self::Invalid("too many arguments".to_string()));
                            }
                        }
                    }
                }

                Ok(Self::Search {
                    pattern,
                    path: path.unwrap_or_else(|| ".".to_string()),
                    case_insensitive,
                })
            }
        }
    }
}

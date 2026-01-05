#[derive(Debug, PartialEq)]
pub enum Command {
    Search { pattern: String, path: String },
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
                let path = parts
                    .get(1)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| ".".to_string());
                Ok(Self::Search { pattern, path })
            }
        }
    }
}

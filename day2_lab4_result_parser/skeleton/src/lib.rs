#[derive(Debug, PartialEq)]
pub enum Command {
    Set { target: String, level: u8 },
    Stop { target: String },
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    MissingField(&'static str),
    UnknownAction(String),
    InvalidLevel(String),
}

pub fn parse_command(line: &str) -> Result<Command, CommandError> {
    let _ = line;
    todo!("parse SET <target> <level> or STOP <target>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_set() {
        assert_eq!(
            parse_command("SET motor 3"),
            Ok(Command::Set { target: "motor".to_string(), level: 3 })
        );
    }

    #[test]
    fn parses_stop() {
        assert_eq!(
            parse_command("STOP motor"),
            Ok(Command::Stop { target: "motor".to_string() })
        );
    }

    #[test]
    fn rejects_unknown_action() {
        assert_eq!(
            parse_command("PAUSE motor"),
            Err(CommandError::UnknownAction("PAUSE".to_string()))
        );
    }

    #[test]
    fn rejects_invalid_level() {
        assert_eq!(
            parse_command("SET motor high"),
            Err(CommandError::InvalidLevel("high".to_string()))
        );
    }
}

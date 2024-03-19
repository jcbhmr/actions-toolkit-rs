use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use crate::utils::to_command_value;

pub type CommandProperties = HashMap<String, String>;

pub fn issue_command(
    command: &str,
    properties: CommandProperties,
    message: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::new(command, properties, message);
    println!("{cmd}");
    Ok(())
}

pub fn issue(name: &str, message: Option<String>) -> Result<(), Box<dyn Error>> {
    issue_command(name, CommandProperties::new(), message)?;
    Ok(())
}

const CMD_STRING: &str = "::";

struct Command {
    command: String,
    properties: CommandProperties,
    message: Option<String>,
}

impl Command {
    pub fn new(command: &str, properties: CommandProperties, message: Option<String>) -> Self {
        let command = match command {
            "" => "missing.command",
            _ => command,
        };
        Self {
            command: command.into(),
            properties,
            message,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cmd_str = CMD_STRING.to_string() + &self.command;
        if !self.properties.is_empty() {
            cmd_str.push(' ');
            let mut first = true;
            for (key, value) in self.properties {
                if first {
                    first = false;
                } else {
                    cmd_str.push(',');
                }
                cmd_str.push_str(&format!("{key}={}", escape_property(Some(value.into()))));
            }
        }
        cmd_str.push_str(&format!("{CMD_STRING}{}", escape_data(self.message)));
        write!(f, "{cmd_str}")
    }
}

fn escape_data(s: Option<String>) -> String {
    to_command_value(s)
        .replace("%", "%25")
        .replace("\r", "%0D")
        .replace("\n", "%0A")
}

fn escape_property(s: Option<String>) -> String {
    to_command_value(s)
        .replace("%", "%25")
        .replace("\r", "%0D")
        .replace("\n", "%0A")
        .replace(":", "%3A")
        .replace(",", "%2C")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command() {
        issue_command(
            "test",
            CommandProperties::from([("key", "value")]),
            Some("message".into()),
        )
        .unwrap();
    }
}

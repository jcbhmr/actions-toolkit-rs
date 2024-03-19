use std::collections::HashMap;
use std::fmt::Display;
use std::error::Error;

pub type CommandProperties = HashMap<String, String>;

pub fn issue_command(command: &str, properties: &CommandProperties, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::new(command, properties, message);
    println!("{cmd}");
    Ok(())
}

const CMD_STRING: &str = "::";

struct Command {
    command: String,
    properties: CommandProperties,
    message: String,
}

impl Command {
    pub fn new(command: &str, properties: &CommandProperties, message: &str) -> Self {
        let command = match command {
            "" => "missing.command",
            _ => command,
        };
        let properties = properties.clone();
        let message = message.into();
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
            for (key, value) in &self.properties {
                if first {
                    first = false;
                } else {
                    cmd_str.push(',');
                }
                cmd_str.push_str(&format!("{key}={}", escape_property(&value)));
            }
        }
        cmd_str.push_str(&format!("{CMD_STRING}{}", escape_data(&self.message)));
        write!(f, "{cmd_str}")
    }
}

fn escape_data(s: &str) -> String {
    s.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
}

fn escape_property(s: &str) -> String {
    s.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A").replace(":", "%3A").replace(",", "%2C")
}
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct TaskConfig {
    commands: HashMap<String, String>,
}

impl TaskConfig {
    fn from_file(file_path: &str) -> Result<Self> {
        let file_content = std::fs::read_to_string(file_path)?;
        let config: TaskConfig = toml::from_str(&file_content)?;
        Ok(config)
    }

    fn execute_command(&self, command_name: &str) {
        if let Some(command) = self.commands.get(command_name) {
            let mut parts = command.split_whitespace();
            let command_name = parts.next().unwrap();
            let args: Vec<_> = parts.collect();

            let status = Command::new(command_name)
                .args(args)
                .status()
                .expect("Failed to execute command.");

            if !status.success() {
                eprintln!("Error executing command: {}", command);
            }
        } else {
            eprintln!("Task not found: {}", command_name);
        }
    }
}

fn main() {
    // inline the config template "template.toml"
    let tmplt = include_str!("template.toml");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <task>", args[0]);
        return;
    }

    let command_name = &args[1];

    // Adjust the file path based on your project structure
    let config_file_path = "Tasks.toml";

    if command_name == "init" {
        std::fs::write(config_file_path, tmplt).expect("Unable to write file");
        return;
    }

    if let Ok(config) = TaskConfig::from_file(config_file_path) {
        config.execute_command(command_name);
    } else {
        eprintln!(
            "Error reading or parsing configuration file: {}",
            config_file_path
        );
    }
}

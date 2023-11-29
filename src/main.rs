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

    fn execute_command(&self, command_name: &str, args: &[String]) -> Result<()> {
        let command = self.commands.get(command_name);

        let command = match command {
            Some(command) => command,
            None => return Err(anyhow::anyhow!("Task not found")),
        };

        // split the command into command and args
        let command = command.split(" ").collect::<Vec<&str>>();
        let command_root = command[0];
        let command_args = &command[1..];

        let args = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        let args = [command_args, &args].concat();

        println!("Running command: {} {}", command_root, args.join(" "));

        let status = Command::new(command_root).args(args).status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Command failed"));
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    // inline the config template "template.toml"
    let tmplt = include_str!("template.toml");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("{}", args.len());
        return Err(anyhow::anyhow!("No task provided. Usage: run <task>"));
    }

    let command_name = &args[1];

    // Adjust the file path based on your project structure
    let config_file_path = "Tasks.toml";

    if command_name == "init" {
        std::fs::write(config_file_path, tmplt)?;
        return Ok(());
    }

    TaskConfig::from_file(config_file_path)?.execute_command(command_name, &args[2..])
}

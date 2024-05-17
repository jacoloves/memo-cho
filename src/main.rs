use std::{fs, path::PathBuf};

use clap::{Arg, ArgAction, Command};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    memodir: String,
    memotmp: String,
    editor: String,
}

fn create_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut config_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    config_dir.push(".cofig/memo-cho");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config_path = dirs::home_dir().ok_or("Could not find home directory")?;
    config_path.push(".config/memo-cho/config.yaml");
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_str)?;
    Ok(config)
}

fn main() {
    let config = load_config().expect("Failed to load config");

    let app = Command::new("memo-cho")
        .version("0.1.0")
        .author("Shotaro Tanaka")
        .about("CLI Memo Tool")
        .subcommand(
            Command::new("new").about("Create a new memo").arg(
                Arg::new("title")
                    .short('t')
                    .long("title")
                    .help("Sets the title of the memo")
                    .required(true)
                    .action(ArgAction::Set),
            ),
        )
        .subcommand(
            Command::new("edit").about("Edits an exisitng memo").arg(
                Arg::new("filename")
                    .help("The filename of the memo to edit")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("delete").about("Deletes a memo").arg(
                Arg::new("filename")
                    .help("The filename of the memo to delete")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(Command::new("list").about("Lists all emmos"))
        .subcommand(
            Command::new("grep").about("Searches memos").arg(
                Arg::new("pattern")
                    .help("Ther search pattern")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(Command::new("serve").about("Serves memos as a web page"));

    let mathes = app.get_matches();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config_test() -> Result<(), Box<dyn std::error::Error>> {
        let loaded_config = load_config()?;

        assert_eq!(loaded_config.memodir, "$HOME/tmp/memo-cho");
        assert_eq!(loaded_config.memotmp, "$HOME/tmp/memo-cho/template");
        assert_eq!(loaded_config.editor, "nvim");

        Ok(())
    }
}

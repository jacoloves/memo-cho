use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process::Command as SysCommand,
};

use clap::Arg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    memodir: String,
    template: String,
    editor: String,
}

fn create_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut config_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    config_dir.push(".config/memo-cho");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_dir = create_config_dir()?;
    create_initial_config_file(config_dir.clone())?;
    let mut config_path = config_dir.clone();
    config_path.push("config.yaml");
    let config_str = fs::read_to_string(config_path)?;
    let mut config: Config = serde_yaml::from_str(&config_str)?;

    config.memodir = replace_home_placeholder(&config.memodir);
    config.template = replace_home_placeholder(&config.template);
    config.editor = replace_home_placeholder(&config.editor);

    Ok(config)
}

fn create_initial_config_file(config_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config_file_path = config_dir.join("config.yaml");

    if config_file_path.exists() {
        return Ok(());
    }

    let home_dir = dirs::home_dir().ok_or("Could not find home direcotry")?;
    let home_dir_str = home_dir
        .to_str()
        .ok_or("Home direcotry path is not valid UTF-8")?;

    let contents = format!(
        "memodir: {home}\ntemplate: {home}/template.md\neditor: nano\n",
        home = home_dir_str
    );

    let mut file = File::create(config_file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn create_memo(config: &Config, title: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("{}-{}.md", date, title);
    let memo_path = PathBuf::from(&config.memodir).join(filename);

    if memo_path.exists() {
        return Err("Memo already exists".into());
    }

    let template_path = PathBuf::from(&config.template);
    if !template_path.exists() {
        return Err("Template file does not exist".into());
    }

    println!("Creating memo at {:?}", &config.template);
    println!("Creating memo at {:?}", &template_path);

    fs::copy(&template_path, &memo_path)?;

    SysCommand::new(&config.editor)
        .arg(
            memo_path
                .to_str()
                .ok_or("Path to string coversion failed")?,
        )
        .spawn()?
        .wait()?;

    Ok(memo_path)
}

fn replace_home_placeholder(path: &str) -> String {
    if let Some(home_dir) = dirs::home_dir() {
        path.replace("$HOME", home_dir.to_str().unwrap())
    } else {
        path.to_string()
    }
}

fn main() {
    let config = load_config().expect("Failed to load config");

    let app = clap::Command::new("memo-cho")
        .version("0.1.0")
        .author("Shotaro Tanaka")
        .about("CLI Memo Tool")
        .subcommand(clap::Command::new("new").about("Create a new memo"))
        .subcommand(clap::Command::new("n").about("Create a new memo (short alias"))
        .subcommand(
            clap::Command::new("edit")
                .about("Edits an exisitng memo")
                .arg(
                    Arg::new("filename")
                        .help("The filename of the memo to edit")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            clap::Command::new("delete").about("Deletes a memo").arg(
                Arg::new("filename")
                    .help("The filename of the memo to delete")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(clap::Command::new("list").about("Lists all emmos"))
        .subcommand(
            clap::Command::new("grep").about("Searches memos").arg(
                Arg::new("pattern")
                    .help("Ther search pattern")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(clap::Command::new("serve").about("Serves memos as a web page"));

    let matches = app.get_matches();

    if matches.subcommand_matches("new").is_some() || matches.subcommand_matches("n").is_some() {
        println!("Title:");
        let mut title = String::new();
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read line");
        let title = title.trim();

        match create_memo(&config, title) {
            Ok(path) => println!("Memo created at {:?}", path),
            Err(e) => eprintln!("Error createting memo: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_config_test() -> Result<(), Box<dyn std::error::Error>> {
        let loaded_config = load_config()?;

        assert_eq!(loaded_config.memodir, "$HOME/tmp/memo-cho");
        assert_eq!(loaded_config.template, "$HOME/tmp/memo-cho/template.md");
        assert_eq!(loaded_config.editor, "nvim");

        Ok(())
    }
}

use clap::{Arg, ArgAction, Command};

fn main() {
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

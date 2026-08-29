use clap::Parser;
use glob::glob;
use std::{
    env,
    fs::{OpenOptions, read_to_string},
    io::{self, Error, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

mod command;
mod ui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, requires = "name")]
    add: Option<String>,

    #[arg(short, long, requires = "add")]
    name: Option<String>,
}

struct Command {
    name: String,
    command: String,
    file_path: PathBuf,
}

impl Command {
    fn to_store_string(&self) -> String {
        format!("{} | {}", self.name, self.command,)
    }
}

static APP_NAME: &str = "this";

fn main() {
    let local_dir = dirs::data_local_dir();
    let dir: PathBuf = match local_dir {
        Some(mut local) => {
            local.push(APP_NAME);
            std::fs::create_dir_all(&local).expect("Could not create the app local directory.");
            local
        }
        None => {
            eprintln!("Unable to find local directory.");
            return;
        }
    };

    let mut global_file = dir.clone();
    global_file.push("global.this");

    let args = Args::parse();

    if let (Some(add_val), Some(name_val)) = (args.add, args.name) {
        let new_command = Command {
            name: add_val.clone(),
            command: name_val,
            file_path: global_file.clone(),
        };

        let append_attempt = add_to_file(&new_command, &global_file);

        match append_attempt {
            Ok(_) => {
                println!("Added command: \n{}", new_command.to_store_string())
            }
            Err(error) => {
                eprintln!("Something went wrong: {}", error)
            }
        }
    } else {
        let mut files_to_parse: Vec<PathBuf> = Vec::new();

        files_to_parse.push(global_file.clone());
        files_to_parse.append(&mut search_directory().unwrap_or_default());

        let mut commands: Vec<Command> = Vec::new();

        for file_path in files_to_parse {
            let res = parse_file(&file_path);

            if let Ok(file_commands) = res {
                for command in file_commands {
                    commands.push(command);
                }
            }
        }

        if commands.is_empty() {
            println!("No commands found.");
            return;
        }

        match ui::run(&commands) {
            Ok(Some(command)) => {
                println!("Running: {}\n", command);

                if cfg!(target_os = "windows") {
                    std::process::Command::new("cmd")
                        .args(["/C", &command])
                        .status()
                        .expect("Failed to execute command");
                } else {
                    std::process::Command::new("sh")
                        .args(["-c", &command])
                        .status()
                        .expect("Failed to execute command");
                };
            }
            Ok(None) => println!("No command selected."),
            Err(err) => eprintln!("Error rendering TUI: {}", err),
        }
    }
}

fn search_directory() -> io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();

    let mut pattern = env::current_dir()?;

    pattern.push("*.this");

    match glob(&pattern.to_string_lossy()) {
        Ok(found) => {
            for entry in found.flatten() {
                paths.push(entry)
            }
        }
        Err(error) => return Err(Error::other(error)),
    }

    Ok(paths)
}

fn add_to_file(command: &Command, file_path: &Path) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    let mut last_byte = [0u8; 1];
    let ends_with_newline = match file.seek(SeekFrom::End(-1)) {
        Ok(_) => {
            file.read_exact(&mut last_byte)?;
            last_byte[0] == b'\n'
        }
        Err(_) => true,
    };

    file.seek(SeekFrom::End(0))?;

    if !ends_with_newline {
        writeln!(file)?;
    }

    writeln!(file, "{}", command.to_store_string())
}

fn parse_file(path: &Path) -> io::Result<Vec<Command>> {
    let raw_text = read_to_string(path)?;

    let mut commands: Vec<Command> = Vec::new();

    for line in raw_text.lines() {
        let sections: Vec<&str> = line.splitn(2, " | ").collect();

        if sections.len() != 2 {
            continue;
        }

        commands.push(Command {
            name: sections[0].to_string(),
            command: sections[1].to_string(),
            file_path: path.to_path_buf(),
        });
    }

    Ok(commands)
}

use std::{ffi::OsStr, fs, process::{Command, exit}};
#[cfg(target_os = "windows")]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use toml::de::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileOptions {
    txt: Option<String>,
    pdf: Option<String>,
    mp4: Option<String>,
    mp3: Option<String>,
    c: Option<String>,
    cpp: Option<String>,
    rs: Option<String>,
    png: Option<String>,
    jpg: Option<String>,
    svg: Option<String>,
    sh: Option<String>,
}

pub fn read_file_options() -> FileOptions {
    #[cfg(target_os = "windows")]
    let config_path = PathBuf::from("D:\\Applications\\columbus\\file_options.toml");
    #[cfg(target_os = "linux")]
    {
        use std::env;

        let mut config_path: PathBuf;

        if let Ok(home_path) = env::var_os("HOME") {
            config_path = PathBuf::from(home_path);
            config_path.push(".config/columbus/file_options.toml");
        }
    }
    if config_path.exists() {
        if let Ok(contents) = fs::read_to_string(&config_path) {
            let file_res: Result<FileOptions, Error> = toml::from_str(&contents);
            if let Ok(options) = file_res {
                return options;
            }
        }
    }
    FileOptions {
        txt: None,      // notepad
        pdf: None,      // start msedge 
        mp4: None,
        mp3: None,
        c: None,
        cpp: None,
        rs: None,
        png: None,
        jpg: None,
        svg: None,
        sh: None,
    }
}

pub fn handle_file_open(file: &PathBuf, options: FileOptions) {
    if let Some(file_ext) = file.extension() {
        match String::from(file_ext.to_string_lossy()).as_str() {
            "txt" => {
                if let Some(command) = options.txt {
                    execute_command(command, file);
                }
            },
            "pdf" => {
                if let Some(command) = options.pdf {
                    execute_command(command, file);
                }
            },
            "mp4" => {
                if let Some(command) = options.mp4 {
                    execute_command(command, file);
                }
            },
            "mp3" => {
                if let Some(command) = options.mp3 {
                    execute_command(command, file);
                }
            },
            "c" => {
                if let Some(command) = options.c {
                    execute_command(command, file);
                }
            },
            "cpp" => {
                if let Some(command) = options.cpp {
                    execute_command(command, file);
                }
            },
            "rs" => {
                if let Some(command) = options.rs {
                    execute_command(command, file);
                }
            },
            "png" => {
                if let Some(command) = options.png {
                    execute_command(command, file);
                }
            },
            "jpg" => {
                if let Some(command) = options.jpg {
                    execute_command(command, file);
                }
            },
            "svg" => {
                if let Some(command) = options.svg {
                    execute_command(command, file);
                }
            },
            "sh" => {
                if let Some(command) = options.sh {
                    execute_command(command, file);
                }
            },
            _ => {}
        }
    } 
}

fn split_command(cmd: String) -> (String, Vec<String>) {
    let cmd_split: Vec<&str> = cmd.splitn(2, " ").collect();
    let mut args = vec![];
    if cmd_split.len() != 1 {
        args = cmd_split[1].split(" ").map(|x| String::from(x)).collect();
    }
    return (String::from(cmd_split[0]), args);
}

pub fn execute_command(command: String, file: &PathBuf) {
    let (cmd, mut args) = split_command(command);
    if let Some(file_path) = file.to_str() {
        args.push(String::from(file_path));
        if let Err(err) = Command::new(cmd).args(args).spawn() {
            // Handle error
            // print!("Error during executing command: {}", err);
        }
    }
}
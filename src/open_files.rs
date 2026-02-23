use std::{fs, process::{Command}};
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
    toml: Option<String>,
    html: Option<String>,
    css: Option<String>,
    js: Option<String>,
    jsx: Option<String>,
    ts: Option<String>,
    tsx: Option<String>,
    py: Option<String>,
    md: Option<String>,
    gitignore: Option<String>,
    rb: Option<String>,
    java: Option<String>,
    kt: Option<String>,
    json: Option<String>,
    zig: Option<String>,
    odin: Option<String>,
}

pub fn read_file_options(config_path: Option<PathBuf>) -> FileOptions {
    let config_path = match config_path {
        Some(input_config_path ) => input_config_path,
        None => {
            #[cfg(target_os = "windows")]
            {
                PathBuf::from("D:\\Applications\\columbus\\file_options.toml")
            }
            #[cfg(target_os = "linux")]
            {
                use std::env;

                let mut file_config_path: PathBuf;

                if let Ok(home_path) = env::var_os("HOME") {
                    file_config_path = PathBuf::from(home_path);
                    file_config_path.push(".config/columbus/file_options.toml");
                }
                file_config_path
            }
        }
    };
    
    if config_path.exists() {
        if let Ok(contents) = fs::read_to_string(&config_path) {
            let file_res: Result<FileOptions, Error> = toml::from_str(&contents);
            if let Ok(options) = file_res {
                return options;
            }
        }
    }
    FileOptions {
        txt: None,
        pdf: None,
        mp4: None,
        mp3: None,
        c: None,
        cpp: None,
        rs: None,
        png: None,
        jpg: None,
        svg: None,
        sh: None,
        toml: None,
        html: None,
        css: None,
        js: None,
        jsx: None,
        ts: None,
        tsx: None,
        py: None,
        md: None,
        gitignore: None,
        rb: None,
        java: None,
        kt: None,
        json: None,
        zig: None,
        odin: None,
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
            "html" => {
                if let Some(command) = options.html {
                    execute_command(command, file);
                }
            },
            "css" => {
                if let Some(command) = options.css {
                    execute_command(command, file);
                }
            },
            "js" => {
                if let Some(command) = options.js {
                    execute_command(command, file);
                }
            },
            "jsx" => {
                if let Some(command) = options.jsx {
                    execute_command(command, file);
                }
            },
            "ts" => {
                if let Some(command) = options.ts {
                    execute_command(command, file);
                }
            },
            "tsx" => {
                if let Some(command) = options.tsx {
                    execute_command(command, file);
                }
            },
            "md" => {
                if let Some(command) = options.md {
                    execute_command(command, file);
                }
            },
            "py" => {
                if let Some(command) = options.py {
                    execute_command(command, file);
                }
            },
            "java" => {
                if let Some(command) = options.java {
                    execute_command(command, file);
                }
            },
            "kt" => {
                if let Some(command) = options.kt {
                    execute_command(command, file);
                }
            },
            "odin" => {
                if let Some(command) = options.odin {
                    execute_command(command, file);
                }
            },
            "zig" => {
                if let Some(command) = options.zig {
                    execute_command(command, file);
                }
            },
            "gitignore" => {
                if let Some(command) = options.gitignore {
                    execute_command(command, file);
                }
            },
            "rb" => {
                if let Some(command) = options.rb {
                    execute_command(command, file);
                }
            },
            "json" => {
                if let Some(command) = options.json {
                    execute_command(command, file);
                }
            },
            "toml" => {
                if let Some(command) = options.toml {
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
        if let Err(_err) = Command::new(cmd).args(args).spawn() {
            // Handle error
            // print!("Error during executing command: {}", err);
        }
    }
}
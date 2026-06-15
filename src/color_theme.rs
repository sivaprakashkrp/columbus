use std::{fs, path::PathBuf};
use toml::de::Error;
use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Default, Clone)]
pub struct ColorTheme {
    pub primary: Color,
    pub header: Color,
    pub border: Color,
    pub selector: Color,
}

#[derive(Debug, Deserialize, Clone)]
struct ThemeConfig {
    pub primary: String,
    pub header: String,
    pub border: String,
    pub selector: String,
}

fn read_color_theme(config_path: Option<PathBuf>) -> ThemeConfig {
    let config_path = match config_path {
        Some(input_config_path ) => input_config_path,
        None => {
            #[cfg(target_os = "windows")]
            {
                PathBuf::from("D:\\Applications\\columbus\\color_theme.toml")
            }
            #[cfg(target_os = "linux")]
            {
                let mut file_config_path: PathBuf;
                if let Ok(home_path) = env::var("XDG_CONFIG_HOME") {
                    file_config_path = PathBuf::from(home_path);
                } else {
                    file_config_path = PathBuf::from(".");
                }
                file_config_path.push("columbus/color_theme.toml");
                file_config_path
            }
        }
    };
    
    if config_path.exists() {
        if let Ok(contents) = fs::read_to_string(&config_path) {
            let file_res: Result<ThemeConfig, Error> = toml::from_str(&contents);
            if let Ok(options) = file_res {
                return options;
            }
        }
    }
    ThemeConfig {
        primary: String::from("#00f0ff"),
        header: String::from("#0000ff"),
        border: String::from("#ffd700"),
        selector: String::from("#00f0ff"),
    }
}

pub fn get_color_theme(config_path: Option<PathBuf>) -> ColorTheme {
    let read_theme = read_color_theme(config_path);
    ColorTheme {
        primary: read_theme.primary.parse().unwrap_or(Color::Cyan),
        header: read_theme.header.parse().unwrap_or(Color::Blue),
        border: read_theme.border.parse().unwrap_or(Color::Yellow),
        selector: read_theme.selector.parse().unwrap_or(Color::Cyan),
    }
}
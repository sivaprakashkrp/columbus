use std::{fs, path::{Path, PathBuf}};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, Cell, HighlightSpacing, Row, ScrollbarState, Table, TableState
    },
};
use serde::{Deserialize, Serialize};
use toml::de::Error;

use crate::{App, dependencies::HandlesInput};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAFileEntry {
    pub name: String,
    pub path: PathBuf,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredQAEntity {
    files: Vec<QAFileEntry>,
}

pub struct QuickAccess {
    pub entries: Vec<QAFileEntry>,
    pub state: TableState,
    pub scroll_state: ScrollbarState,
    pub in_focus: bool,
}

impl QAFileEntry {
    fn ref_array(&self) -> [String; 1] {
        [
            self.name.clone(),
        ]
    }
}

const ITEM_HEIGHT: usize = 1;
impl QuickAccess {
    pub fn new() -> QuickAccess {
        const ITEM_HEIGHT: usize = 1;
        let data_vec = get_qa_files();
        QuickAccess {
            entries: data_vec.clone(),
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((&data_vec.len() - 1) * ITEM_HEIGHT),
            in_focus: false,
        }
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn create_qa_entries_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default().fg(Color::Black).bg(Color::Blue);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Cyan);
        let selected_col_style = Style::default().fg(Color::Yellow);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Blue);

        let header = ["File Name"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.entries.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => Color::from_u32(0x00001122),
                _ => Color::from_u32(0x00112233),
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("{content}"))))
                .collect::<Row>()
                .style(Style::new().fg(Color::Cyan).bg(color))
                .height(1)
        });
        let bar = " ▶ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Min(100),
            ],
        ).block(
            Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(" Quick Access ")
            .border_style(
                if self.in_focus {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }
            )
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![bar.into(),]))
        .bg(Color::Black)
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }
}

pub fn get_qa_files() -> Vec<QAFileEntry> {
    #[cfg(target_os = "windows")]
    let qa_path = PathBuf::from("D:\\Applications\\columbus\\qa_files.toml");
    #[cfg(target_os = "linux")]
    {
        use std::env;

        let mut qa_path: PathBuf;

        if let Ok(home_path) = env::var_os("HOME") {
            qa_path = PathBuf::from(home_path);
            qa_path.push(".config/columbus/qa_files.toml");
        }
        
    }
    
    if qa_path.exists() {
        if let Ok(contents) = fs::read_to_string(qa_path.clone()) {
            let file_res: Result<StoredQAEntity, Error> = toml::from_str(&contents);
            if let Ok(files) = file_res {
                let mut read_files = files.files;
                read_files.sort_by_key(|a| a.count);
                read_files.reverse();
                let max_limit = std::cmp::min(read_files.len(), 20);
                return read_files[0..max_limit].to_vec();
            }
        }
    }

    vec![
        QAFileEntry {name: String::from("Columbus_QA"), path: PathBuf::from(qa_path.parent().unwrap_or(Path::new("."))), count: 0}
    ]
}

pub fn update_qa_files(app: &mut App, file_name: String, path: PathBuf) {
    let array = app.quick_access.entries.clone();
    let iterator = array.iter().enumerate();
    let mut flag = true;
    let input_path = if path.is_dir() {path} else {PathBuf::from(path.parent().unwrap_or(Path::new(&path)))} ;
    for (i, entry) in iterator {
        if !entry.path.exists() {
            app.quick_access.entries.remove(i);
        }
        if entry.path == input_path {
            app.quick_access.entries[i].count += 1;
            flag = false;
            break;
        }
    }
    if flag {
        if input_path.is_absolute() && input_path.parent().map_or(true, |p| p == input_path) {
            app.quick_access.entries.push(QAFileEntry { name: String::from(input_path.to_string_lossy()), path: input_path, count: 1 })
        } else {
            app.quick_access.entries.push(QAFileEntry { name: file_name.clone(), path: input_path, count: 1 })
        }
    }
}

pub fn write_qa_data(app: &mut App) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let qa_path = PathBuf::from("D:\\Applications\\columbus\\qa_files.toml");
    #[cfg(target_os = "linux")]
    {
        use std::env;

        let mut qa_path: PathBuf;

        if let Ok(home_path) = env::var_os("HOME") {
            qa_path = PathBuf::from(home_path);
            qa_path.push(".config/columbus/qa_files.toml");
        }
    }

    let to_write_str = StoredQAEntity { files: app.quick_access.entries.clone() };

    match toml::to_string(&to_write_str) {
        Ok(content) => {
            if let Err(_res) = fs::write(qa_path, content) {
                return Err(String::from("Error in writing qa data"));
            }
        },
        Err(err) => {
            return Err(String::from(format!("Error in creating qa data to write, {err}")));
        }
    }
    Ok(())
}

impl HandlesInput for QuickAccess {
    fn handle_input(&mut self, event: Event) -> Result<(), String> {
        match event {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                            KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                            _ => ()
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
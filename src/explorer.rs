use std::path::PathBuf;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState,
    },
};

use crate::dependencies::delete;
use crate::{
    dependencies::{HandlesInput, copy_directory, copy_file},
    file_deps::get_data,
};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub e_type: EntryType,
    pub name: String,
    pub size: String,
    pub modified_at: String,
    pub hidden: bool,
    pub is_exec: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryType {
    File,
    Dir,
}

pub struct Explorer {
    pub root_path: PathBuf,
    pub include_hidden: bool,
    pub copy_src_path: Option<PathBuf>,
    pub files: Vec<FileEntry>,
    pub copied_item: Option<EntryType>,
    pub file_is_cut: bool,
    pub state: TableState,
    pub scroll_state: ScrollbarState,
    pub in_focus: bool,
}

impl FileEntry {
    fn ref_array(&self) -> [String; 4] {
        let type_of_entry = format!("{:?}", self.e_type);
        [
            type_of_entry,
            self.name.clone(),
            self.size.clone(),
            self.modified_at.clone(),
        ]
    }
}

const ITEM_HEIGHT: usize = 1;
impl Explorer {
    pub fn new(path: &PathBuf, include_hidden: bool) -> Explorer {
        const ITEM_HEIGHT: usize = 1;
        if let Ok(data_vec) = get_data(path, include_hidden, false, false, false) {
            return Explorer {
                root_path: PathBuf::from(path),
                include_hidden: include_hidden,
                files: data_vec.clone(),
                copy_src_path: None,
                copied_item: None,
                file_is_cut: false,
                state: TableState::default().with_selected(0),
                scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
                in_focus: true,
            };
        } else {
            return Explorer {
                root_path: PathBuf::from(path),
                include_hidden: include_hidden,
                files: vec![],
                state: TableState::default().with_selected(0),
                scroll_state: ScrollbarState::new(ITEM_HEIGHT),
                in_focus: true,
                copy_src_path: None,
                copied_item: None,
                file_is_cut: false,
            };
        }
    }

    pub fn refresh(&mut self, path: &PathBuf, include_hidden: bool) {
        self.root_path = path.clone();
        if let Ok(data_vec) = get_data(path, include_hidden, false, false, false) {
            self.files = data_vec;
        } else {
            self.files = vec![];
        }
        self.state.select(Some(0));
        self.scroll_state = self.scroll_state.position(0 * ITEM_HEIGHT);
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
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
                    self.files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn create_explorer_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default().fg(Color::Black).bg(Color::Blue);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Cyan);
        let selected_col_style = Style::default().fg(Color::Yellow);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Blue);

        let header = ["Type", "Name", "Size", "Modified At"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.files.iter().enumerate().map(|(i, data)| {
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
        // let bar = " █ ";
        let bar = " ▶ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(5),
                Constraint::Min(50),
                Constraint::Min(10),
                Constraint::Min(15),
            ],
        )
        .block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Explorer ")
                .border_style(if self.in_focus {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }),
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![bar.into()]))
        .bg(Color::Black)
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }

    pub fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn handle_copy(&mut self) {
        if let Some(idx) = self.state.selected() {
            let mut file_path = self.root_path.clone();
            file_path.push(&self.files[idx].name);
            self.copy_src_path = Some(file_path);
            if self.files[idx].e_type == EntryType::File {
                self.copied_item = Some(EntryType::File)
            } else {
                self.copied_item = Some(EntryType::Dir)
            }
        }
    }

    fn handle_paste(&mut self) {
        if self.copy_src_path != None {
            match self.copied_item.clone() {
                Some(file_type) => {
                    let mut paste_path = self.root_path.clone();
                    if let Some(src_file_path) = self.copy_src_path.clone() {
                        match src_file_path.file_name() {
                            Some(file_name) => {
                                let mut file_name_str =
                                    String::from(file_name.to_str().unwrap_or("default"));
                                loop {
                                    let mut flag: bool = false;
                                    for entry in &self.files {
                                        if entry.name == file_name_str {
                                            file_name_str.insert_str(0, "Copy-");
                                            flag = true;
                                        }
                                    }
                                    if !flag {
                                        break;
                                    }
                                }
                                paste_path.push(file_name_str)
                            }
                            None => {}
                        }
                        if file_type.to_owned() == EntryType::File {
                            if let Ok(_suc) = copy_file(&src_file_path, &paste_path) {
                                // Handle success
                            }
                        } else {
                            if let Ok(_suc) = copy_directory(&src_file_path, &paste_path) {
                                // Handle success
                            }
                        }
                        if self.file_is_cut == true {
                            delete(&src_file_path, file_type.to_owned());
                            self.file_is_cut = false;
                        }
                        self.refresh(&self.root_path.clone(), self.include_hidden);
                    }
                }
                None => {}
            }
        }
    }

    fn handle_delete(&mut self) {
        if let Some(idx) = self.state.selected() {
            let mut file_path = self.root_path.clone();
            file_path = file_path.join(self.files[idx].name.clone());
            delete(&file_path, self.files[idx].e_type.clone());
        }
        self.refresh(&self.root_path.clone(), self.include_hidden);
    }
}

impl HandlesInput for Explorer {
    fn handle_input(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                            KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                            KeyCode::Char('r') => {
                                self.refresh(&self.root_path.clone(), self.include_hidden);
                            }
                            KeyCode::Delete => self.handle_delete(),
                            KeyCode::Char('c') => self.handle_copy(),
                            KeyCode::Char('v') => self.handle_paste(),
                            KeyCode::Char('x') => {
                                self.handle_copy();
                                self.file_is_cut = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

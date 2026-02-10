use std::{fs::{self, remove_dir_all, remove_file}, path::PathBuf};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{Frame, layout::{Constraint, Margin, Rect}, style::{Color, Modifier, Style, Stylize}, text::Text, widgets::{Block, Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState}};

use crate::{App, dependencies::HandlesInput, file_deps::get_data};

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
    pub files: Vec<FileEntry>,
    pub state: TableState,
    pub scroll_state: ScrollbarState,
    pub in_focus: bool,
}

impl FileEntry {
    fn ref_array(&self) -> [String; 4] {
        let type_of_entry = format!("{:?}", self.e_type);
        [type_of_entry, self.name.clone(), self.size.clone(), self.modified_at.clone()]
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
                state: TableState::default().with_selected(0),
                scroll_state: ScrollbarState::new((data_vec.len()-1) * ITEM_HEIGHT),
                in_focus: true,
            };
        } else {
            return Explorer { root_path: PathBuf::from(path), include_hidden: include_hidden, files: vec![], state: TableState::default().with_selected(0), scroll_state: ScrollbarState::new(ITEM_HEIGHT), in_focus: true,}
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
        let header_style = Style::default()
            .fg(Color::Black)
            .bg(Color::Blue);
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
        ).block(
            Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(" Explorer ")
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
        .highlight_symbol(Text::from(vec![
            bar.into(),
        ]))
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
                            KeyCode::Delete => {
                                if let Some(idx) = self.state.selected() {
                                    let mut file_path = self.root_path.clone();
                                    file_path = file_path.join(self.files[idx].name.clone());
                                    if self.files[idx].e_type == EntryType::Dir {
                                        if let Ok(suc) = remove_dir_all(&file_path) {};
                                    } else if self.files[idx].e_type == EntryType::File {
                                        if let Ok(suc) = fs::remove_file(file_path) {};
                                    }
                                }
                                self.refresh(&self.root_path.clone(), self.include_hidden);
                            },
                            KeyCode::Char('c') => {
                                // Copying a file into clipboard
                            },
                            KeyCode::Char('v') => {
                                // Pasting the file in clipboard into the directory in explorer
                            },
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
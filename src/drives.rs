use std::path::PathBuf;

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
use sysinfo::Disks;

use crate::{dependencies::HandlesInput,};

#[derive(Debug, Clone)]
pub struct DriveEntry {
    pub name: String,
    pub mount_point: PathBuf,
}

pub struct Drives {
    pub drives: Vec<DriveEntry>,
    pub state: TableState,
    pub scroll_state: ScrollbarState,
    pub in_focus: bool,
}

impl DriveEntry {
    fn ref_array(&self) -> [String; 2] {
        [
            self.name.clone(),
            String::from(self.mount_point.to_string_lossy()),
        ]
    }
}

const ITEM_HEIGHT: usize = 1;
impl Drives {
    pub fn new() -> Drives {
        const ITEM_HEIGHT: usize = 1;
        let data_vec = get_drives();
        Drives {
            drives: data_vec.clone(),
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((&data_vec.len() - 1) * ITEM_HEIGHT),
            in_focus: false,
        }
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.drives.len() - 1 {
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
                    self.drives.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn refresh(&mut self) {
        self.drives = get_drives();
        self.state.select(Some(0));
        self.scroll_state = self.scroll_state.position(0 * ITEM_HEIGHT);
    }

    pub fn create_drives_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default().fg(Color::Black).bg(Color::Blue);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Cyan);
        let selected_col_style = Style::default().fg(Color::Yellow);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(Color::Blue);

        let header = ["Drive", "Mount"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.drives.iter().enumerate().map(|(i, data)| {
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
                Constraint::Min(90),
                Constraint::Min(10),
            ],
        ).block(
            Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(" Drives ")
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
        .highlight_symbol(Text::from(vec![bar.into()]))
        .bg(Color::Black)
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }
}

fn get_drives() -> Vec<DriveEntry> {
    let disks = Disks::new_with_refreshed_list();
    let mut res: Vec<DriveEntry> = vec![];
    for disk in &disks {
        res.push(DriveEntry {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: PathBuf::from(disk.mount_point()),
        });
    }
    res
}

impl HandlesInput for Drives {
    fn handle_input(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                            KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                            KeyCode::Char('r') => {
                                self.refresh();
                            },
                            _ => ()
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
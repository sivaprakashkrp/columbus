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

use crate::{dependencies::HandlesInput};

#[derive(Debug, Clone)]
pub struct QAFileEntry {
    pub name: String,
    pub path: PathBuf,
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
        let data_vec = get_files();
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

pub fn get_files() -> Vec<QAFileEntry> {
    vec![
        QAFileEntry { name: String::from("Hello there"), path: PathBuf::from(".")}
    ]
}

impl HandlesInput for QuickAccess {
    fn handle_input(&mut self, event: Event) {
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
    }
}
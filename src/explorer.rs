use std::path::PathBuf;

use ratatui::{Frame, layout::{Constraint, Margin, Rect}, style::{Color, Modifier, Style, Stylize}, text::Text, widgets::{Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState}};

use crate::file_deps::get_data;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub e_type: EntryType,
    pub name: String,
    pub size: String,
    pub modified_at: String,
    pub hidden: bool,
    pub is_exec: bool,
}

#[derive(Debug, Clone)]
pub enum EntryType {
    File,
    Dir,
}

pub struct Explorer {
    pub files: Vec<FileEntry>,
    pub state: TableState,
    pub scroll_state: ScrollbarState,
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
                files: data_vec.clone(),
                state: TableState::default().with_selected(0),
                scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            };
        } else {
            return Explorer { files: vec![], state: TableState::default().with_selected(0), scroll_state: ScrollbarState::new(ITEM_HEIGHT)}
        }
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

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
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
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(Color::Cyan).bg(color))
                .height(3)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(5),
                Constraint::Min(20),
                Constraint::Min(20),
                Constraint::Min(20),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            "".into(),
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
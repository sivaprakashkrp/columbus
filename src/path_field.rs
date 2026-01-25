use std::{path::PathBuf};
use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::{Line, Span}, widgets::{Widget}};

use crate::dependencies::FocusableWidget;

pub struct PathField {
    pub path: PathBuf,
    pub path_str: String,
    pub is_focused: bool,
}

impl Widget for &PathField {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        // let display = format!("{} {}", "Path:", self.path_str);
        // buf.set_string(area.x, area.y, display, Style::default())
        Line::from(vec![
            Span::styled("Path: ", Style::default().fg(ratatui::style::Color::Cyan).bold()),
            Span::raw(&self.path_str)
        ]).render(area, buf);
    }
}

impl FocusableWidget for &PathField {
    fn on_focus(&self) -> bool {
        return self.is_focused;
    }
}
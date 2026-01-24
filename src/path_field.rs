use std::{path::PathBuf};
use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::{Line, Span}, widgets::{Widget}};

pub struct PathField {
    pub path: PathBuf,
    pub path_str: String,
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
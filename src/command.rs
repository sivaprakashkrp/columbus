use ratatui::{style::{Style}, text::{Line, Span}, widgets::{Widget}};

use crate::dependencies::FocusableWidget;

pub struct Command {
    pub input: String,
    pub is_focused: bool,
}

impl Widget for &Command {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) where Self: Sized {
        Line::from(vec![
            Span::styled("Command: ", Style::default().fg(ratatui::style::Color::Cyan).bold()),
            Span::raw(&self.input)
        ]).render(area, buf);
    }
}

impl FocusableWidget for &Command {
    fn on_focus(&self) -> bool {
        return self.is_focused;
    }
}


// let instructions = Line::from(vec![
//     " <Tab>".blue().bold(),
//     " Change Focus ".into(),
//     "<H>".blue().bold(),
//     " Detailed Help ".into(),
//     "<Q>".blue().bold(),
//     " Quit ".into(),
// ]).centered();

// Paragraph::new("Command: ").block(Block::bordered().border_type(BorderType::Rounded).title_bottom(instructions)).render(area, buf);
use ratatui::{style::Stylize, text::Line, widgets::{Block, BorderType, Paragraph, Widget}};

pub struct Command {
    pub input: String
}

impl Widget for &Command {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) where Self: Sized {
        let instructions = Line::from(vec![
            " <Tab>".blue().bold(),
            " Change Focus ".into(),
            "<H>".blue().bold(),
            " Detailed Help ".into(),
            "<Q>".blue().bold(),
            " Quit ".into(),
        ]).centered();

        Paragraph::new("Command: ").block(Block::bordered().border_type(BorderType::Rounded).title_bottom(instructions)).render(area, buf);
    }
}
use ratatui::{Frame, layout::Rect, style::Style, widgets::{Paragraph}};

pub struct LogPanel {
    pub msg: String,
}

impl LogPanel {
    pub fn new() -> LogPanel {
        LogPanel {
            msg: String::from(""),
        }
    }

    pub fn set_log(&mut self, msg: String) {
        self.msg = msg;
    }

    pub fn clear_log(&mut self) {
        self.msg = String::from("");
    }

    pub fn render_widget(&self, frame: &mut Frame, area: Rect) {
        let log_panel = Paragraph::new(format!("Log: {}", self.msg))
            .style(Style::default());
        
        frame.render_widget(log_panel, area);
    }
}
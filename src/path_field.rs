use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use std::path::PathBuf;
use tui_input::{Input, InputRequest, backend::crossterm::EventHandler};

use crate::{dependencies::{HandlesInput, InputMode}};

#[derive(Debug, Default, Clone)]
pub struct PathField {
    /// Current value of the input box
    pub input: Input,
    /// Current input mode
    pub input_mode: InputMode,
    pub in_focus: bool,
}

impl PathField {
    pub fn new(path: &PathBuf) -> PathField {
        PathField {
            input: Input::new(String::from(path.to_string_lossy())),
            input_mode: InputMode::Normal,
            in_focus: false,
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.input.reset();
        for c in value.chars() {
            self.input.handle(InputRequest::InsertChar(c));
        }
    }

    pub fn render_input(&self, frame: &mut Frame, area: Rect) {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Cyan.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(" Path ")
                    .border_style(
                        if self.in_focus {
                            Style::default().fg(Color::Cyan)
                        } else {
                            Style::default()
                        }
                    )
            );
        
        frame.render_widget(input, area);

        if self.input_mode == InputMode::Editing {
            // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
            // end of the input text and one line down from the border to the input line
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }
}

impl HandlesInput for PathField {
    fn handle_input(&mut self, event: Event) -> Result<(), String> {
        match event {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    if self.input_mode == InputMode::Normal {
                        match key_event.code {
                            KeyCode::Char('a') => self.input_mode = InputMode::Editing,
                            _ => {}
                        }
                    } else {
                        match key_event.code {
                            KeyCode::Esc => self.input_mode = InputMode::Normal,
                            _ => {
                                self.input.handle_event(&Event::Key(key_event));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

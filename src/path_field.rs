use std::{path::PathBuf};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, buffer::Buffer, layout::Rect, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Paragraph, Widget}};
use tui_input::{Input, backend::crossterm::EventHandler};

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

#[derive(Debug, Default)]
pub struct PathFieldFS {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

impl PathFieldFS {
    pub fn new(path: &PathBuf) -> PathFieldFS {
        PathFieldFS {
            input: Input::new(String::from(path.to_string_lossy())),
            input_mode: InputMode::Normal,
        }
    }

    pub fn start_editing(&mut self) {
        self.input_mode = InputMode::Editing;
        loop {
            if let Ok(event) = event::read() {
                if let Event::Key(key) = event {
                    match key.code {
                        KeyCode::Enter => (),
                        KeyCode::Esc => {
                            self.stop_editing();
                            break;
                        },
                        _ => {
                            self.input.handle_event(&event);
                        }
                    }
                }
            }
        }
    }

    pub fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal
    }

    pub fn render_input(&self, frame: &mut Frame, area: Rect) {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().border_type(ratatui::widgets::BorderType::Rounded).title(" Path "));
        frame.render_widget(input, area);

        if self.input_mode == InputMode::Editing {
            // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
            // end of the input text and one line down from the border to the input line
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }
}
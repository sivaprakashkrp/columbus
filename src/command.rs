use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, layout::Rect, style::{Color, Style, Stylize}, text::{Line}, widgets::{Block, Paragraph}};
use tui_input::{Input, backend::crossterm::EventHandler};

pub struct Command {
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

impl Command {
    pub fn new() -> Command {
        Command {
            input: Input::new(String::from("")),
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
        let instructions = Line::from(vec![
            " <Tab>".blue().bold(),
            " Change Focus ".into(),
            "<H>".blue().bold(),
            " Detailed Help ".into(),
            "<Q>".blue().bold(),
            " Quit ".into(),
        ]).centered();
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().border_type(ratatui::widgets::BorderType::Rounded).title(" Command ").title_bottom(instructions));
        frame.render_widget(input, area);

        if self.input_mode == InputMode::Editing {
            // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
            // end of the input text and one line down from the border to the input line
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }
}

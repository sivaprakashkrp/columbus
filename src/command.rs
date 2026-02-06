use std::{fs::File, path::PathBuf, thread::sleep, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, layout::Rect, style::{Color, Style, Stylize}, text::{Line}, widgets::{Block, Paragraph}};
use tui_input::{Input, InputRequest, backend::crossterm::EventHandler};

use crate::{App, dependencies::{HandlesInput, InputMode}};

pub struct Command {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    pub input_mode: InputMode,
    pub in_focus: bool,
}

impl Command {
    pub fn new() -> Command {
        Command {
            input: Input::new(String::from("")),
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
            .block(
                Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(" Command ")
                .title_bottom(instructions)
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

pub fn handle_command_enter(app: &mut App) {
    let cmd = app.command.input.value();
    let split_cmd: Vec<&str> = cmd.split(" ").collect();
    if split_cmd[0] == "n" {
        let dir_path = PathBuf::from(app.path_field.input.value());
        let mut new_file = dir_path.clone();
        new_file.push(split_cmd[1]);
        if let Err(err) = File::create_new(new_file) {
            app.command.set_value(String::from("The file already exists"));
        } else {
            app.command.set_value(String::from("The file created successfully"));
        }
        app.explorer.refresh(&dir_path, app.include_hidden);
        sleep(Duration::from_secs(1));
        app.command.input.reset();
    }
}

impl HandlesInput for Command {
    fn handle_input(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    if self.input_mode == InputMode::Normal {
                        match key_event.code {
                            KeyCode::Char('a') => self.input_mode = InputMode::Editing,
                            _ => {
                            }
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
    }
}

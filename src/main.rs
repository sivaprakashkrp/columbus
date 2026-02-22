use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, layout::{Constraint, Direction, Layout, Rect}, style::Stylize, widgets::{Block, BorderType, Paragraph}
};
use std::{env::current_dir, path::{Path, PathBuf}, sync::mpsc, thread::{self}};
use strum::{EnumIter, IntoEnumIterator};

mod command;
mod dependencies;
mod drives;
mod explorer;
mod file_deps;
mod file_size_deps;
mod path_field;
mod quick_access;
mod open_files;
mod log_panel;
mod help_overview;
use crate::{
    command::{Command, handle_command_enter}, dependencies::{HandlesInput, InputMode, focus_to, focus_toggler}, drives::Drives, explorer::{Explorer, explorer_handle_enter}, help_overview::HelpOverview, log_panel::LogPanel, path_field::PathField, quick_access::{QuickAccess, update_qa_files, write_qa_data}
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum CurrentWidget {
    Explorer,
    PathField,
    Drives,
    QuickAccess,
    CommandBar,
}

impl CurrentWidget {
    fn next(&self) -> Self {
        let variants: Vec<CurrentWidget> = CurrentWidget::iter().collect();
        let current_index = variants.iter().position(|&v| v == *self).unwrap();
        let next_index = (current_index + 1) % variants.len();
        variants[next_index]
    }
}

pub struct App {
    exit: bool,
    quick_access: QuickAccess,
    path_field: PathField,
    command: Command,
    explorer: Explorer,
    drives: Drives,
    focus_on: CurrentWidget,
    include_hidden: bool,
    log_panel: LogPanel,
    help_overview: HelpOverview,
    help_shown: bool,
}

#[derive(Debug, Parser)]
#[command(
    version,
    author,
    about = "A TUI File Explorer",
    long_about = "A TUI File Explorer written in Rust using Ratatui.\n\nFor Help about keybindings and commands that can be used within columbus, press <H> in the app to open Help Overview.\n\nYou can also refer to the repository at https://github.com/sivaprakashkrp/columbus for more details and help.\n\nIf you encounter any issues, please report them at the Github page of as mentioned above.\n\nEnjoy exploring with columbus!!\n
    ",
    help_template = "{bin} {version}\nDeveloped By: {author}\n\n{about}\n\nUsage:\n\t{usage}\n\n{all-args}",
    author = "Sivaprakash P"
)]
struct CLI {
    path: Option<PathBuf>,
    #[arg(
        short = 'a',
        long = "include-hidden",
        help = "Includes hidden files and folders"
    )]
    include_hidden: bool,
    #[arg(
        short = 'c',
        long = "config",
        help = "Path to file_options.toml file"
    )]
    file_options_path: Option<PathBuf>,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> Result<(), String> {
        while !self.exit {
            if let Ok(rec_event) = rx.recv() {
                match rec_event {
                    Event::Key(key_event) => {
                        if key_event.kind == KeyEventKind::Press {
                            // Clearing log output before handling the operation
                            self.log_panel.clear_log();
                            if self.help_shown {
                                match key_event.code {
                                    KeyCode::Char('q') => self.help_shown = false,
                                    KeyCode::Char('j') | KeyCode::Down => self.help_overview.scroll = if self.help_overview.scroll >= self.help_overview.max_scroll {self.help_overview.max_scroll} else {self.help_overview.scroll + 1},
                                    KeyCode::Char('k') | KeyCode::Up => self.help_overview.scroll = self.help_overview.scroll.saturating_sub(1),
                                    _ => (),
                                }
                            } else {
                                match key_event.code {
                                    KeyCode::Tab => {
                                        focus_toggler(self);
                                        self.focus_on = self.focus_on.next();
                                        focus_toggler(self);
                                    },
                                    KeyCode::Enter => {
                                        match self.focus_on {
                                            CurrentWidget::PathField => {
                                                let mut input_path = PathBuf::from(self.path_field.input.value());
                                                if input_path.exists() {
                                                    if !input_path.is_dir() {
                                                        input_path = PathBuf::from(input_path.parent().unwrap_or(Path::new(".")));
                                                    }
                                                    self.path_field.set_value(String::from(input_path.to_string_lossy()));
                                                    self.explorer.refresh(&input_path, self.include_hidden);
                                                    focus_to(self, CurrentWidget::Explorer);
                                                }
                                            },
                                            CurrentWidget::Explorer => {
                                                explorer_handle_enter(self);
                                            }
                                            CurrentWidget::Drives => {
                                                if let Some(selected_idx) = self.drives.state.selected() {
                                                    let entry = &self.drives.drives[selected_idx];
                                                    let dir_path = PathBuf::from(entry.mount_point.clone());
                                                    self.path_field.set_value(String::from(dir_path.to_string_lossy()));
                                                    self.explorer.refresh(&dir_path, self.include_hidden);
                                                    focus_to(self, CurrentWidget::Explorer);
                                                } else {}
                                            },
                                            CurrentWidget::CommandBar => {
                                                handle_command_enter(self);
                                            },
                                            CurrentWidget::QuickAccess => {
                                                if let Some(selected_idx) = self.quick_access.state.selected() {
                                                    let entry = &self.quick_access.entries[selected_idx];
                                                    let dir_path = PathBuf::from(entry.path.clone());
                                                    self.path_field.set_value(String::from(dir_path.to_string_lossy()));
                                                    self.explorer.refresh(&dir_path, self.include_hidden);
                                                    self.quick_access.state.select(Some(0));
                                                    focus_to(self, CurrentWidget::Explorer);
                                                } else {}
                                            }
                                        }
                                        update_qa_files(self, String::from(PathBuf::from(self.path_field.input.value()).file_name().and_then(|name| name.to_str()).unwrap_or("default")), PathBuf::from(self.path_field.input.value()));
                                    },
                                    KeyCode::Char(':') => {
                                        if self.focus_on == CurrentWidget::CommandBar && self.command.input_mode == InputMode::Editing  || self.focus_on == CurrentWidget::PathField && self.path_field.input_mode == InputMode::Editing {
                                            self.get_focused_widget().handle_input(rec_event)?;
                                        } else {
                                            focus_to(self, CurrentWidget::CommandBar);
                                            self.command.input_mode = InputMode::Editing;
                                        }
                                    },
                                    KeyCode::Char('a') => {
                                        if self.focus_on == CurrentWidget::CommandBar && self.command.input_mode == InputMode::Editing  || self.focus_on == CurrentWidget::PathField && self.path_field.input_mode == InputMode::Editing {
                                            self.get_focused_widget().handle_input(rec_event)?;
                                        } else {
                                                focus_to(self, CurrentWidget::PathField);
                                                self.path_field.input_mode = InputMode::Editing;
                                        }
                                    },
                                    KeyCode::Backspace => {
                                        if self.focus_on == CurrentWidget::CommandBar && self.command.input_mode == InputMode::Editing  || self.focus_on == CurrentWidget::PathField && self.path_field.input_mode == InputMode::Editing {
                                            self.get_focused_widget().handle_input(rec_event)?;
                                        } else {
                                            let current_dir = PathBuf::from(self.path_field.input.value());
                                            if let Some(parent_dir) = current_dir.parent() {
                                                let parent_dir_str = String::from(parent_dir.to_string_lossy());
                                                self.path_field.set_value(parent_dir_str);
                                                self.explorer.refresh(&PathBuf::from(parent_dir), self.include_hidden);
                                            } else {}
                                        }
                                    },
                                    KeyCode::Char('h') => self.help_shown = true,
                                    KeyCode::Char('q') => {
                                        if self.focus_on == CurrentWidget::CommandBar && self.command.input_mode == InputMode::Editing  || self.focus_on == CurrentWidget::PathField && self.path_field.input_mode == InputMode::Editing {
                                            self.get_focused_widget().handle_input(rec_event)?;
                                        } else {
                                            self.exit_app();
                                        }
                                    },
                                    _ => self.get_focused_widget().handle_input(rec_event)?,
                                }
                            }
                        } else {
                            self.get_focused_widget().handle_input(rec_event)?;
                        }
                    }
                    _ => self.get_focused_widget().handle_input(rec_event)?,
                }
            }
            terminal.draw(|frame| self.draw(frame)).expect("Unable to draw to the terminal");
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        // Creating the Layout Blocks
        let vertical_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Percentage(90),
            Constraint::Length(3),
            Constraint::Length(1),
        ]);
        let vertical_split_areas = vertical_layout.split(frame.area());

        let [title, path_bar] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(vertical_split_areas[0]);

        let [sidebar, explorer_cont_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(vertical_split_areas[1]);

        let [explorer_area, explorer_scroll_bar] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(97), Constraint::Percentage(3)])
            .areas(explorer_cont_area);

        let [drive_area, quick_access_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .areas(sidebar);

        // Rendering the Title
        frame.render_widget(
            Paragraph::new("COLUMBUS")
                .centered()
                .block(Block::bordered().border_type(BorderType::Rounded))
                .bold()
                .cyan(),
            title,
        );

        // Rendering the PathField Widget
        self.path_field.render_input(frame, path_bar);

        // Rendering the Command area
        self.command.render_input(frame, vertical_split_areas[2]);

        // Rendering the explorer area
        self.explorer.create_explorer_table(frame, explorer_area);
        self.explorer.render_scrollbar(frame, explorer_scroll_bar);

        // Rendering the drives area
        self.drives.create_drives_table(frame, drive_area);

        // Rendering the quick access area
        self.quick_access.create_qa_entries_table(frame, quick_access_area);

        // Rendering the Log Panel
        self.log_panel.render_widget(frame, vertical_split_areas[3]);      

        // Conditionally rendering the help overview
        if self.help_shown {
            let area = frame.area();

            let help_popup_area = Rect {
                x: area.width / 10,
                y: area.height / 10,
                width: (0.8 * area.width as f32) as u16,
                height: (0.8 * area.height as f32) as u16,
            };

            self.help_overview.render(help_popup_area, frame.buffer_mut());
        }
    }

    fn get_focused_widget(&mut self) -> &mut dyn HandlesInput {
        if self.focus_on == CurrentWidget::CommandBar {
            return &mut self.command;
        } else if self.focus_on == CurrentWidget::Drives {
            return &mut self.drives;
        } else if self.focus_on == CurrentWidget::PathField {
            return &mut self.path_field;
        } else if self.focus_on == CurrentWidget::QuickAccess {
            return &mut self.quick_access;
        }
        return &mut self.explorer;
    }

    fn exit_app(&mut self) {
        if let Err(err) = write_qa_data(self) {
            self.log_panel.set_log(err);
        }
        self.exit = true;
    }
}

fn main() {
    let cli = CLI::parse();

    let mut current_path = match cli.path {
        Some(res) => res,
        None => current_dir().unwrap_or(PathBuf::from(".")),
    };

    current_path = if current_path.is_dir() { current_path } else { PathBuf::from(&current_path.parent().unwrap_or(Path::new(".")))};

    current_path = std::path::absolute(current_path.clone()).unwrap_or(current_path);

    let mut terminal = ratatui::init();

    let mut app: App = App {
        exit: false,
        quick_access: QuickAccess::new(),
        path_field: PathField::new(&current_path),
        command: Command::new(),
        explorer: Explorer::new(&current_path, cli.file_options_path,cli.include_hidden),
        drives: Drives::new(),
        focus_on: CurrentWidget::Explorer,
        include_hidden: cli.include_hidden,
        log_panel: LogPanel::new(),
        help_overview: HelpOverview::new(),
        help_shown: false,
    };

    // Spawning a input thread
    let (tx, rx) = mpsc::channel::<Event>();
    thread::spawn(move || handle_input_events(tx.clone()));

    if let Err(err)  = app.run(&mut terminal, rx) {
        app.log_panel.set_log(err);
    }

    ratatui::restore();
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        if let Ok(rec_event) = crossterm::event::read() {
            if let Ok(_suc) = tx.send(rec_event) {
                // Success of transmission
            }
        }
    }
}

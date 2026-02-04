use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, BorderType, Paragraph},
};
use std::{env::current_dir, io, path::PathBuf, sync::mpsc, thread};
use strum::{EnumIter, IntoEnumIterator};

mod command;
mod dependencies;
mod drives_deps;
mod explorer;
mod file_deps;
mod file_size_deps;
mod path_field;
use crate::{
    command::Command,
    dependencies::{HandlesInput},
    drives_deps::Drives,
    explorer::Explorer,
    path_field::PathField,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum CurrentWidget {
    PathField,
    CommandBar,
    Explorer,
    QuickAccess,
    Drives,
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
}

struct QuickAccess {}

#[derive(Debug, Parser)]
#[command(
    version,
    author,
    about = "A TUI File Explorer",
    long_about = "<Long About Comes here>",
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
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            if let Ok(rec_event) = rx.recv() {
                match rec_event {
                    Event::Key(key_event) => {
                        if key_event.kind == KeyEventKind::Press {
                            match key_event.code {
                                KeyCode::Char('q') => self.exit = true,
                                _ => self.get_focused_widget().handle_input(rec_event),
                            }
                        } else {
                            self.get_focused_widget().handle_input(rec_event);
                        }
                    }
                    _ => self.get_focused_widget().handle_input(rec_event),
                }
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        // Creating the Layout Blocks
        let vertical_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Percentage(90),
            Constraint::Length(3),
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
        // render_widget(frame, &self.path_field, path_bar);
        self.path_field.render_input(frame, path_bar);

        // Rendering the Command area
        // render_widget(frame, &self.command, vertical_split_areas[2]);
        self.command.render_input(frame, vertical_split_areas[2]);

        // Rendering the explorer area
        self.explorer.create_explorer_table(frame, explorer_area);
        self.explorer.render_scrollbar(frame, explorer_scroll_bar);

        // Rendering the drives area
        self.drives.create_drives_table(frame, drive_area);
    }

    // fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
    //     // if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
    //     //     self.exit = true;
    //     // } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Tab {
    //     //     focus_toggler(self);
    //     //     self.focused_widget = self.focused_widget.next();
    //     //     focus_toggler(self);
    //     // }

    //     if key_event.kind == KeyEventKind::Press {
    //         match key_event.code {
    //             KeyCode::Char('q') => self.exit = true,
    //             KeyCode::Char('j') | KeyCode::Down => self.explorer.next_row(),
    //             KeyCode::Char('k') | KeyCode::Up => self.explorer.previous_row(),
    //             KeyCode::Char('l') | KeyCode::Right => self.explorer.next_column(),
    //             KeyCode::Char('h') | KeyCode::Left => self.explorer.previous_column(),
    //             KeyCode::Char('e') => self.path_field.start_editing(),
    //             KeyCode::Esc => self.path_field.stop_editing(),
    //             // KeyCode::Tab => {
    //             //     focus_toggler(self);
    //             //     self.focused_widget = self.focused_widget.next();
    //             //     focus_toggler(self);
    //             // },
    //             _ => ()
    //         }
    //     }

    //     Ok(())
    // }

    fn get_focused_widget(&mut self) -> impl HandlesInput {
        self.path_field.clone()
    }
}

// impl Widget for &App {
//     fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) where Self: Sized {
//

//         // Rendering the Title
//         Paragraph::new("COLUMBUS").centered().block(Block::bordered().border_type(BorderType::Rounded)).bold().cyan().render(title, buf);

//         // Rendering the Path Bar
//         //let path_text = Line::from(format!("Path: {}", self.path));
//         //Paragraph::new(path_text).block(Block::bordered().border_type(BorderType::Rounded)).render(path_bar, buf);
//         // self.path_field.render(path_bar, buf, self.path_field);

//         // Rendering the Quick-Access Area
//         Block::bordered().border_type(BorderType::Rounded).title(" Quick Access ").render(quick_access_area, buf);

//         // Rendering the drives Area
//         Block::bordered().border_type(BorderType::Rounded).title(" Drives ").render(drive_area, buf);

//         // Rendering Explorer Area
//         Block::bordered().border_type(BorderType::Rounded).title(" Explorer ").render(explorer_area, buf);

//         // Rendering Instructions
//         let instructions = Line::from(vec![
//             " <Tab>".blue().bold(),
//             " Change Focus ".into(),
//             "<H>".blue().bold(),
//             " Detailed Help ".into(),
//             "<Q>".blue().bold(),
//             " Quit ".into(),
//         ]).centered();
//         Paragraph::new("Command: ").block(Block::bordered().border_type(BorderType::Rounded).title_bottom(instructions)).render(command_area, buf);

//     }
// }

fn main() -> io::Result<()> {
    let cli = CLI::parse();

    let current_path = match cli.path {
        Some(res) => res,
        None => current_dir().unwrap_or(PathBuf::from(".")),
    };

    let mut terminal = ratatui::init();

    let mut app: App = App {
        exit: false,
        quick_access: QuickAccess {},
        path_field: PathField::new(&current_path),
        command: Command::new(),
        explorer: Explorer::new(&current_path, cli.include_hidden),
        drives: Drives::new(),
        focus_on: CurrentWidget::PathField,
    };

    // Spawning a input thread
    let (tx, rx) = mpsc::channel::<Event>();
    thread::spawn(move || handle_input_events(tx.clone()));

    let app_result = app.run(&mut terminal, rx);

    ratatui::restore();

    app_result
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        if let Ok(rec_event) = crossterm::event::read() {
            if let Ok(suc) = tx.send(rec_event) {
                // Success of transmission
            }
        }
    }
}

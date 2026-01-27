use std::{env::current_dir, io, path::PathBuf};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout}, style::Stylize, widgets::{Block, BorderType, Paragraph, ScrollbarState, TableState}};
use clap::{Parser};
use strum::{EnumIter, IntoEnumIterator};

mod path_field;
mod command;
mod dependencies;
mod explorer;
mod file_size_deps;
mod file_deps;
use crate::{command::Command, dependencies::{focus_toggler, render_widget}, explorer::Explorer, path_field::PathField};

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
    focused_widget: CurrentWidget,
}

struct QuickAccess {

}

#[derive(Debug, Parser)]
#[command(
    version,
    author,
    about = "A TUI File explorer",
    long_about = "<Long About Comes here>",
    help_template = "{bin} {version}\nDeveloped By: {author}\n\n{about}\n\nUsage:\n\t{usage}\n\n{all-args}",
    author = "Sivaprakash P"
)]
struct CLI {
    path: Option<PathBuf>,
    #[arg(short = 'a', long = "include-hidden", help = "Includes hidden files and folders")]
    include_hidden: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        // Creating the Layout Blocks
        let vertical_layout = Layout::vertical([Constraint::Length(3), Constraint::Percentage(90), Constraint::Length(3)]);
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
        frame.render_widget(Paragraph::new("COLUMBUS").centered().block(Block::bordered().border_type(BorderType::Rounded)).bold().cyan(), title);

        // Rendering the PathField Widget
        render_widget(frame, &self.path_field, path_bar);

        // Rendering the instructions area
        render_widget(frame, &self.command, vertical_split_areas[2]);

        self.explorer.create_explorer_table(frame, explorer_area);
        self.explorer.render_scrollbar(frame, explorer_scroll_bar);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        // if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
        //     self.exit = true;
        // } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Tab {
        //     focus_toggler(self);
        //     self.focused_widget = self.focused_widget.next();
        //     focus_toggler(self);
        // }

        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('j') | KeyCode::Down => self.explorer.next_row(),
                KeyCode::Char('k') | KeyCode::Up => self.explorer.previous_row(),
                KeyCode::Char('l') | KeyCode::Right => self.explorer.next_column(),
                KeyCode::Char('h') | KeyCode::Left => self.explorer.previous_column(),
                KeyCode::Tab => {
                    focus_toggler(self);
                    self.focused_widget = self.focused_widget.next();
                    focus_toggler(self);
                },
                _ => ()
            }
        }
        
        Ok(())
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
        None => current_dir().unwrap_or(PathBuf::from("."))
    };
    
    let path_widget: PathField = PathField {path: current_path.clone(), path_str: current_path.to_str().unwrap_or(".").to_owned(), is_focused: true};

    let mut terminal = ratatui::init();

    let mut app: App = App {
        exit: false,
        quick_access: QuickAccess {},
        path_field: path_widget,
        command: Command { input: String::new(), is_focused: false },
        explorer: Explorer::new(&current_path, cli.include_hidden),
        focused_widget: CurrentWidget::PathField,
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result
}
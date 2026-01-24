use std::{env::current_dir, io, path::PathBuf};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout, Rect}, style::Stylize, text::{Line, Text}, widgets::{Block, BorderType, Paragraph, Widget}};
use clap::{Parser};

mod path_field;
mod command;
use crate::{command::Command, path_field::PathField};

enum EntryType {
    File,
    Dir,
}

struct FileEntry {
    e_type: EntryType,
    name: String,
    size: String,
    modified_at: String,
    hidden: bool,
}

struct App {
    exit: bool,
    quick_access: QuickAccess,
    path_field: PathField,
    command: Command,
    explorer: Explorer,
}

struct QuickAccess {

}

struct Explorer {
    files: Vec<FileEntry>,
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

    fn draw(&self, frame: &mut Frame) {
        // Creating the Layout Blocks
        let vertical_layout = Layout::vertical([Constraint::Length(3), Constraint::Percentage(90), Constraint::Length(3)]);
        let vertical_split_areas = vertical_layout.split(frame.area()); 

        let [title, path_bar] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(vertical_split_areas[0]);

        let [sidebar, explorer_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(vertical_split_areas[1]);

        let [drive_area, quick_access_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .areas(sidebar);


        // Rendering the Title
        frame.render_widget(Paragraph::new("COLUMBUS").centered().block(Block::bordered().border_type(BorderType::Rounded)).bold().cyan(), title);

        // Rendering the PathField Widget
        render_widget(frame, &self.path_field, path_bar);

        // Rendering the instructions area
        frame.render_widget(&self.command, vertical_split_areas[2]);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
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
    
    let path_widget: PathField = PathField {path: current_path.clone(), path_str: current_path.to_str().unwrap_or(".").to_owned()};

    let mut terminal = ratatui::init();

    let mut app: App = App {
        exit: false,
        quick_access: QuickAccess {},
        path_field: path_widget,
        command: Command { input: String::new() },
        explorer: Explorer { files: Vec::new() },
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result
}

fn render_widget(frame: &mut Frame, widget: impl Widget, area: Rect) {
    let cont_block = Block::bordered().border_type(BorderType::Rounded);
    frame.render_widget(&cont_block, area);
    frame.render_widget(widget, cont_block.inner(area));
}
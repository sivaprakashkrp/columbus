use std::{fs::{self, remove_dir_all, remove_file}, path::PathBuf};

use fs_extra::dir::{CopyOptions, copy};
use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Widget},
};

use crate::{App, CurrentWidget, explorer::EntryType};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

pub trait FocusableWidget {
    fn on_focus(&self) -> bool;
}

pub fn render_widget(frame: &mut Frame, widget: impl Widget + FocusableWidget, area: Rect) {
    let mut cont_block = Block::bordered().border_type(BorderType::Rounded);
    if widget.on_focus() {
        cont_block = cont_block.border_style(Color::Cyan);
    }
    frame.render_widget(&cont_block, area);
    frame.render_widget(widget, cont_block.inner(area));
}

pub fn focus_toggler(app: &mut App) {
    match app.focus_on {
        CurrentWidget::CommandBar => {
            app.command.in_focus = !app.command.in_focus;
            if !app.command.in_focus {
                app.command.input_mode = InputMode::Normal
            }
        }
        CurrentWidget::Explorer => app.explorer.in_focus = !app.explorer.in_focus,
        CurrentWidget::PathField => {
            app.path_field.in_focus = !app.path_field.in_focus;
            if !app.path_field.in_focus {
                app.path_field.input_mode = InputMode::Normal;
            }
        }
        CurrentWidget::QuickAccess => app.quick_access.in_focus = !app.quick_access.in_focus,
        CurrentWidget::Drives => app.drives.in_focus = !app.drives.in_focus,
    }
}

pub fn focus_to(app: &mut App, widg: CurrentWidget) {
    focus_toggler(app);
    app.focus_on = widg;
    focus_toggler(app);
}

pub trait HandlesInput {
    fn handle_input(&mut self, event: crossterm::event::Event);
}

pub fn copy_file(src: &PathBuf, dest: &PathBuf) -> Result<u64, String> {
    if let Ok(_success) = fs::copy(src, dest) {
        Ok(_success)
    } else {
        Err(String::from(format!(
            "Error in copying file {}",
            src.to_str().unwrap()
        )))
    }
}

pub fn copy_directory(src: &PathBuf, dest: &PathBuf) -> Result<(), String> {
    let options = CopyOptions::new().copy_inside(true);
    if let Ok(_copied_size) = copy(src, dest, &options) {
        Ok(())
    } else {
        Err(String::from("The Folder was not copied successfully."))
    }
}

pub fn delete(file_path: &PathBuf, file_type: EntryType) {
    if file_type == EntryType::Dir {
        if let Ok(suc) = remove_dir_all(&file_path) {};
    } else if file_type == EntryType::File {
        if let Ok(suc) = remove_file(file_path) {};
    }
}

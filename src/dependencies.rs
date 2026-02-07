use std::path::PathBuf;

use ratatui::{Frame, layout::Rect, style::{Color}, widgets::{Block, BorderType, Widget}};

use crate::{App, CurrentWidget};

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
        },
        CurrentWidget::Explorer => app.explorer.in_focus = !app.explorer.in_focus,
        CurrentWidget::PathField => {
            app.path_field.in_focus = !app.path_field.in_focus;
            if !app.path_field.in_focus {
                app.path_field.input_mode = InputMode::Normal;
            }
        },
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
use ratatui::{Frame, layout::Rect, style::{Color}, widgets::{Block, BorderType, Widget}};

use crate::{App, CurrentWidget};

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
    match app.focused_widget {
        CurrentWidget::CommandBar => {
            app.command.is_focused = !app.command.is_focused;
        },
        CurrentWidget::Explorer => {},
        CurrentWidget::PathField => {
            // app.path_field.is_focused = !app.path_field.is_focused;
            // app.path_field.run();
        },
        CurrentWidget::QuickAccess => {},
        CurrentWidget::Drives => {},    
    }
}
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
    match app.focus_on {
        CurrentWidget::CommandBar => {},
        CurrentWidget::Explorer => {},
        CurrentWidget::PathField => {},
        CurrentWidget::QuickAccess => {},
        CurrentWidget::Drives => {},    
    }
}

pub trait HandlesInput {
    fn handle_input(&mut self, event: crossterm::event::Event);
}
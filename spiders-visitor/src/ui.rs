use ratatui::{widgets::Paragraph, Frame};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
    let greeting = Paragraph::new(format!("Spiders TUI for {}", app.title));
    frame.render_widget(greeting, frame.area());
}

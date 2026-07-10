use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, status: &str) {
    let footer =
        Paragraph::new(format!("STATUS: {}", status)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}

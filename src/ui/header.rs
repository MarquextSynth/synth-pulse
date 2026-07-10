use ratatui::{
    Frame,
    widgets::{Block, Borders},
};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect) {
    let header = Block::default()
        .title(" SYNTH-PULSE ")
        .borders(Borders::ALL);

    frame.render_widget(header, area);
}

use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    synth::SynthPulse,
    ui::{footer, header, layout},
    widgets::logo::LOGO,
};

pub fn render(frame: &mut Frame, app: &SynthPulse) {
    let ui = layout::split(frame);

    header::render(frame, ui.header);

    let body = Paragraph::new(LOGO)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(body, ui.body);

    footer::render(frame, ui.footer, &app.status);
}

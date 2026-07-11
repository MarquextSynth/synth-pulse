use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::*,
};

use crate::{synth::SynthPulse, widgets::logo::LOGO};

pub fn render(frame: &mut Frame, synth: &SynthPulse) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // ==========================
    // HEADER
    // ==========================

    let logo = Paragraph::new(LOGO).block(
        Block::default()
            .title(" Synth-Pulse ")
            .borders(Borders::ALL),
    );

    frame.render_widget(logo, chunks[0]);

    // ==========================
    // BODY
    // ==========================

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(chunks[1]);

    // ==========================
    // SEARCH RESULTS
    // ==========================

    let items: Vec<ListItem> = if synth.search_results.is_empty() {
        vec![ListItem::new("Press '/' to start searching")]
    } else {
        synth
            .search_results
            .iter()
            .enumerate()
            .map(|(i, song)| {
                if i == synth.selected {
                    ListItem::new(format!("▶ {}", song.title))
                } else {
                    ListItem::new(format!("  {}", song.title))
                }
            })
            .collect()
    };

    let search = List::new(items).block(Block::default().title(" Search ").borders(Borders::ALL));

    frame.render_widget(search, body[0]);

    // ==========================
    // PLAYBACK HUD
    // ==========================

    let title = synth
        .now_playing
        .clone()
        .unwrap_or_else(|| "Nothing Playing".into());

    // Barra temporal (la animaremos después)
    let progress = "■■■■■■■■□□□□□□□□□□□□";
    let time = "01:43 / 03:58";

    let playback = Paragraph::new(format!(
        "▶ NOW PLAYING\n\n{}\n\nVOL 100%\n\n{}\n{}\n",
        title, progress, time,
    ))
    .block(Block::default().title(" Playback ").borders(Borders::ALL))
    .wrap(Wrap { trim: true });

    frame.render_widget(playback, body[1]);

    // ==========================
    // FOOTER
    // ==========================

    let footer_text = if matches!(synth.mode, crate::synth::AppMode::Search) {
        format!("Search > {}", synth.search_input)
    } else {
        format!(
            "STATUS: {} | Results: {} | / Search | ↑↓ Select | Enter Play | P Radio | Q Exit",
            synth.status,
            synth.search_results.len()
        )
    };

    let footer = Paragraph::new(footer_text)
        .block(Block::default().title(" Console ").borders(Borders::ALL));

    frame.render_widget(footer, chunks[2]);
}

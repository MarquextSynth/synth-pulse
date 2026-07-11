mod player;
mod search;
mod synth;
mod ui;
mod widgets;

use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

use player::{controller::Player, ipc::MpvIpc};
use search::youtube::search;
use synth::{AppMode, SynthPulse};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut synth = SynthPulse::new();
    let mut player = Player::new();

    while synth.running {
        // ============================================
        // Refresh MPV state
        // ============================================

        if synth.playing {
            if let Some(pos) = MpvIpc::playback_time() {
                synth.position = pos;
            }

            if let Some(len) = MpvIpc::duration() {
                synth.duration = len;
            }

            if let Some(vol) = MpvIpc::volume() {
                synth.volume = vol;
            }

            // DEBUG IPC
            synth.status = format!(
                "P:{:.1}s D:{:.1}s V:{:.0}%",
                synth.position, synth.duration, synth.volume
            );
        }

        // ============================================
        // Draw UI
        // ============================================

        terminal.draw(|frame| {
            ui::home::render(frame, &synth);
        })?;

        // ============================================
        // Keyboard
        // ============================================

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match synth.mode {
                    AppMode::Search => match key.code {
                        KeyCode::Esc => {
                            synth.mode = AppMode::Home;
                            synth.status = "READY".into();
                        }

                        KeyCode::Enter => {
                            synth.search_results = search(&synth.search_input);
                            synth.selected = 0;

                            synth.mode = AppMode::Home;
                            synth.status = format!("{} results", synth.search_results.len());
                        }

                        KeyCode::Backspace => {
                            synth.search_input.pop();
                        }

                        KeyCode::Char(c) => {
                            synth.search_input.push(c);
                        }

                        _ => {}
                    },

                    _ => match key.code {
                        KeyCode::Char('q') => {
                            player.stop();

                            synth.playing = false;
                            synth.now_playing = None;

                            synth.quit();
                        }

                        KeyCode::Char('p') => {
                            player.play("https://stream.live.vc.bbcmedia.co.uk/bbc_world_service");

                            synth.playing = true;
                            synth.now_playing = Some("BBC World Service".into());

                            synth.position = 0.0;
                            synth.duration = 0.0;
                        }

                        KeyCode::Char('/') => {
                            synth.search_input.clear();
                            synth.mode = AppMode::Search;
                            synth.status = "SEARCH".into();
                        }

                        KeyCode::Up => {
                            if synth.selected > 0 {
                                synth.selected -= 1;
                            }
                        }

                        KeyCode::Down => {
                            if synth.selected + 1 < synth.search_results.len() {
                                synth.selected += 1;
                            }
                        }

                        KeyCode::Enter => {
                            if let Some(song) = synth.search_results.get(synth.selected) {
                                player.play(&song.url);

                                synth.playing = true;
                                synth.now_playing = Some(song.title.clone());

                                synth.position = 0.0;
                                synth.duration = 0.0;
                            }
                        }

                        _ => {}
                    },
                }
            }
        }
    }

    player.stop();

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.show_cursor()?;

    Ok(())
}

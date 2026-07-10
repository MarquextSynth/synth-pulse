use std::process::{Command, Stdio};

pub struct Player;

impl Player {
    pub fn play(url: &str) {
        let _ = Command::new("mpv")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    }
}

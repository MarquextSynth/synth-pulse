use std::process::{Child, Command, Stdio};

pub struct Player {
    process: Option<Child>,
}

impl Player {
    pub fn new() -> Self {
        Self { process: None }
    }

    pub fn play(&mut self, url: &str) {
        // Si ya hay una reproducción, la detenemos primero.
        self.stop();

        // Elimina un socket antiguo si existe
        let _ = std::fs::remove_file("/tmp/synthpulse.sock");

        match Command::new("mpv")
            .arg("--no-video")
            .arg("--input-ipc-server=/tmp/synthpulse.sock")
            .arg("--force-window=no")
            .arg("--quiet")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => {
                self.process = Some(child);
            }
            Err(err) => {
                eprintln!("Error al lanzar mpv: {err}");
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

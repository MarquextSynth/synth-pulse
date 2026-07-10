pub enum AppMode {
    Home,
    Player,
    Radio,
    Search,
    Settings,
}

pub struct SynthPulse {
    pub running: bool,
    pub mode: AppMode,
    pub status: String,
}

impl SynthPulse {
    pub fn new() -> Self {
        Self {
            running: true,
            mode: AppMode::Home,
            status: String::from("READY"),
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

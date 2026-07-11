use crate::search::models::SearchResult;

pub enum AppMode {
    Home,
    Search,
    Player,
    Radio,
    Settings,
}

pub struct SynthPulse {
    pub running: bool,

    pub mode: AppMode,

    pub status: String,

    pub search_results: Vec<SearchResult>,

    pub selected: usize,

    pub search_input: String,

    pub now_playing: Option<String>,

    pub playing: bool,

    pub position: f64,

    pub duration: f64,

    pub volume: f64,
}

impl SynthPulse {
    pub fn new() -> Self {
        Self {
            running: true,

            mode: AppMode::Home,

            status: "READY".into(),

            search_results: Vec::new(),

            selected: 0,

            search_input: String::new(),

            now_playing: None,

            playing: false,

            position: 0.0,

            duration: 0.0,

            volume: 100.0,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

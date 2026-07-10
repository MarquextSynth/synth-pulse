use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub struct UiLayout {
    pub header: Rect,
    pub body: Rect,
    pub footer: Rect,
}

pub fn split(frame: &Frame) -> UiLayout {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    UiLayout {
        header: areas[0],
        body: areas[1],
        footer: areas[2],
    }
}

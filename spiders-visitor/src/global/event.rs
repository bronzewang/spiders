use rat_salsa::{
    event::{QuitEvent, RenderedEvent},
    timer::TimeOut,
};

pub enum VisEvent {
    #[allow(dead_code)]
    Event(ratatui::crossterm::event::Event),
    #[allow(dead_code)]
    TimeOut(TimeOut),
    Rendered,
    Quit,
}

impl From<RenderedEvent> for VisEvent {
    fn from(_: RenderedEvent) -> Self {
        Self::Rendered
    }
}

impl From<QuitEvent> for VisEvent {
    fn from(_: QuitEvent) -> Self {
        Self::Quit
    }
}

impl From<TimeOut> for VisEvent {
    fn from(value: TimeOut) -> Self {
        Self::TimeOut(value)
    }
}

impl From<ratatui::crossterm::event::Event> for VisEvent {
    fn from(value: ratatui::crossterm::event::Event) -> Self {
        Self::Event(value)
    }
}

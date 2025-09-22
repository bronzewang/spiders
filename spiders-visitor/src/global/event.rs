use rat_salsa::{event::{QuitEvent, RenderedEvent}, timer::TimeOut};

pub enum VEvent {
    #[allow(dead_code)]
    Event(ratatui::crossterm::event::Event),
    #[allow(dead_code)]
    TimeOut(TimeOut),
    Rendered,
    Quit,
}

impl From<RenderedEvent> for VEvent {
    fn from(_: RenderedEvent) -> Self {
        Self::Rendered
    }
}

impl From<QuitEvent> for VEvent {
    fn from(_: QuitEvent) -> Self {
        Self::Quit
    }
}

impl From<TimeOut> for VEvent {
    fn from(value: TimeOut) -> Self {
        Self::TimeOut(value)
    }
}

impl From<ratatui::crossterm::event::Event> for VEvent {
    fn from(value: ratatui::crossterm::event::Event) -> Self {
        Self::Event(value)
    }
}

use anyhow::Error;
use global::VisEvent;
use global::GlobalContext;
use rat_salsa::Control;
use rat_salsa::{
    RunConfig,
    poll::{PollCrossterm, PollQuit, PollRendered, PollTasks, PollTimers},
    run_tui,
};
use ratatui::{buffer::Buffer, layout::Rect};

mod global;

fn main() -> Result<(), Error> {
    let mut context = GlobalContext::new();
    let mut state = SceneryState::default();

    run_tui(
        init,
        render,
        event,
        error,
        &mut context,
        &mut state,
        RunConfig::default()?
            .poll(PollCrossterm)
            .poll(PollTasks::default())
            .poll(PollTimers::default())
            .poll(PollRendered)
            .poll(PollQuit),
    )?;

    Ok(())
}

fn init(
    _state: &mut SceneryState, //
    _context: &mut GlobalContext,
) -> Result<(), Error> {
    Ok(())
}

fn render(
    _area: Rect, //
    _buf: &mut Buffer,
    _state: &mut SceneryState,
    _context: &mut GlobalContext,
) -> Result<(), Error> {
    Ok(())
}

fn event(
    _event: &VisEvent, //
    _state: &mut SceneryState,
    _context: &mut GlobalContext,
) -> Result<Control<VisEvent>, Error> {
    Ok(Control::Continue)
}

fn error(
    _error: Error, //
    _state: &mut SceneryState,
    _context: &mut GlobalContext,
) -> Result<Control<VisEvent>, Error> {
    Ok(Control::Continue)
}

#[derive(Debug, Default)]
struct SceneryState {}

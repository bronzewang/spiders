use anyhow::Error;
use global::GlobalCtx;
use global::VisEvent;
use rat_salsa::Control;
use rat_salsa::{
    RunConfig,
    poll::{PollCrossterm, PollQuit, PollRendered, PollTasks, PollTimers},
    run_tui,
};
use ratatui::{buffer::Buffer, layout::Rect};

mod global;

fn main() -> Result<(), Error> {
    let mut global_ctx = GlobalCtx::new();
    let mut scenery_state = SceneryState::default();

    run_tui(
        init,
        render,
        event,
        error,
        &mut global_ctx,
        &mut scenery_state,
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
    _scenery_state: &mut SceneryState, //
    _global_ctx: &mut GlobalCtx,
) -> Result<(), Error> {
    Ok(())
}

fn render(
    _area: Rect, //
    _buf: &mut Buffer,
    _scenery_state: &mut SceneryState,
    _global_ctx: &mut GlobalCtx,
) -> Result<(), Error> {
    Ok(())
}

fn event(
    _event: &VisEvent, //
    _scenery_state: &mut SceneryState,
    _global_ctx: &mut GlobalCtx,
) -> Result<Control<VisEvent>, Error> {
    Ok(Control::Continue)
}

fn error(
    _error: Error, //
    _scenery_state: &mut SceneryState,
    _global_ctx: &mut GlobalCtx,
) -> Result<Control<VisEvent>, Error> {
    Ok(Control::Continue)
}

#[derive(Debug, Default)]
struct SceneryState {}

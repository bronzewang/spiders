use anyhow::Error;
use global::VEvent;
use global::GlobalCtx;
use rat_salsa::Control;
use rat_salsa::{poll::{PollCrossterm, PollQuit, PollRendered, PollTasks, PollTimers}, run_tui, RunConfig};
use ratatui::{buffer::Buffer, layout::Rect};

mod global;

fn main() -> Result<(), Error>{

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
) -> Result<(), Error>
{
    Ok(())
}

fn render(
    _area: Rect, //
    _buf: &mut Buffer,
    _scenery_state: &mut SceneryState,
    _global_ctx: &mut GlobalCtx,
) -> Result<(), Error>
{
    Ok(())
}

fn event(
    _event: &VEvent, //
    _scenery_state: &mut SceneryState,
    _global_ctx: &mut GlobalCtx,
) -> Result<Control<VEvent>, Error>
{
    Ok(Control::Continue)
}

fn error(
    _error: Error, //
    _scenery_state: &mut SceneryState,
    _global_ctx: &mut GlobalCtx,
) -> Result<Control<VEvent>, Error>
{
    Ok(Control::Continue)
}

#[derive(Debug, Default)]
struct SceneryState {
}

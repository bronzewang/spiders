use anyhow::Error;
use global::VisEvent;
use global::GlobalContext;
use rat_salsa::Control;
use rat_salsa::{
    RunConfig,
    SalsaContext,
    poll::{PollCrossterm, PollQuit, PollRendered, PollTasks, PollTimers},
    run_tui,
};
use rat_widget::event::ct_event;
use rat_widget::event::try_flow;
use rat_widget::focus::FocusBuilder;
use rat_widget::focus::FocusFlag;
use rat_widget::focus::HasFocus;
use rat_widget::menu::MenuBuilder;
use rat_widget::menu::MenuStructure;
use rat_widget::menu::Menubar;
use rat_widget::menu::MenubarState;
use rat_widget::menu::MenubarLine;
use rat_widget::menu::MenubarPopup;
use rat_widget::popup::Placement;
use rat_widget::statusline::StatusLine;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::widgets::Block;
use ratatui::{buffer::Buffer, layout::Rect};
use ratatui::prelude::StatefulWidget;

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
    state: &mut SceneryState, //
    context: &mut GlobalContext,
) -> Result<(), Error> {
    context.set_focus(FocusBuilder::build_for(state));

    state.menu.bar.select(Some(0));
    Ok(())
}

fn render(
    area: Rect, //
    buf: &mut Buffer,
    state: &mut SceneryState,
    context: &mut GlobalContext,
) -> Result<(), Error> {
    let l = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(area);
    let s = Layout::horizontal([
        Constraint::Percentage(69),
        Constraint::Percentage(31),
    ])
    .split(l[1]);

    let menu_struct = Menu {};
    let (menu, menu_popup) = Menubar::new(&menu_struct)
        .title("Spiders")
        .popup_width(25)
        .popup_block(Block::bordered())
        .popup_placement(Placement::Above)
        .into_widgets();
    menu.render(s[0], buf, &mut state.menu);

    // let status = StatusLine::new()
    //     .layout([Constraint::Fill(1), Constraint::Length(14)]);
    // status.render(s[1], buf, &mut state.status);

    Ok(())
}

fn event(
    event: &VisEvent, //
    state: &mut SceneryState,
    context: &mut GlobalContext,
) -> Result<Control<VisEvent>, Error> {
    match event {
        VisEvent::Event(event) => {
            try_flow!(match &event {
                ct_event!(key press CONTROL-'q') => Control::Quit,
                _ => Control::Continue,
            });
        }
        VisEvent::Quit => {
            try_flow!(
                Control::Quit
            );
        }
        _ => {}
    }
    Ok(Control::Continue)
}

fn error(
    _error: Error, //
    _state: &mut SceneryState,
    _context: &mut GlobalContext,
) -> Result<Control<VisEvent>, Error> {
    Ok(Control::Continue)
}

#[derive(Debug)]
struct Menu {
}

impl<'a> MenuStructure<'a> for Menu {
    fn menus(&'a self, menu: &mut MenuBuilder<'a>) {
        menu.item_parsed("_View")
            .item_parsed("_Quit");
    }

    fn submenu(&'a self, n: usize, submenu: &mut MenuBuilder<'a>) {
        match n {
            0 => {
                submenu.item_parsed("_Split view");
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct SceneryState {
    menu: MenubarState,
}

impl Default for SceneryState {
    fn default() -> Self {
        Self {
            menu: MenubarState::named("menu"),
        }
    }
}

impl HasFocus for SceneryState {
    /// Build the focus-structure for the container.
    fn build(&self, _builder: &mut FocusBuilder) {
    }

    /// Access to the flag for the rest.
    fn focus(&self) -> FocusFlag {
        unimplemented!("don't use this")
    }

    /// Area for mouse focus.
    ///
    /// This area shouldn't overlap with areas returned by other widgets.
    /// If it does, the widget should use `area_z()` for clarification.
    /// Otherwise, the areas are searched in order of addition.
    fn area(&self) -> Rect {
        unimplemented!("don't use this")
    }
}

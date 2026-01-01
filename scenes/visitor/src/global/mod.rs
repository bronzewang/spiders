use anyhow::Error;
use rat_salsa::{SalsaAppContext, SalsaContext};

pub use event::VisEvent;

mod event;

#[derive(Debug)]
pub struct GlobalContext {
    context: SalsaAppContext<VisEvent, Error>,
}

impl SalsaContext<VisEvent, Error> for GlobalContext {
    fn set_salsa_ctx(&mut self, app_context: SalsaAppContext<VisEvent, Error>) {
        self.context = app_context;
    }

    fn salsa_ctx(&self) -> &SalsaAppContext<VisEvent, Error> {
        &self.context
    }
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            context: Default::default(),
        }
    }
}

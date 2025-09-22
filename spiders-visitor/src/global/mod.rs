use anyhow::Error;
use rat_salsa::{SalsaAppContext, SalsaContext};

pub use event::VEvent;

mod event;

#[derive(Debug)]
pub struct GlobalCtx {
    ctx: SalsaAppContext<VEvent, Error>,
}

impl SalsaContext<VEvent, Error> for GlobalCtx {
    fn set_salsa_ctx(&mut self, app_ctx: SalsaAppContext<VEvent, Error>) {
        self.ctx = app_ctx;
    }

    fn salsa_ctx(&self) -> &SalsaAppContext<VEvent, Error> {
        &self.ctx
    }
}

impl GlobalCtx {
    pub fn new() -> Self {
        Self {
            ctx: Default::default(),
        }
    }
}

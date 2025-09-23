use anyhow::Error;
use rat_salsa::{SalsaAppContext, SalsaContext};

pub use event::VisEvent;

mod event;

#[derive(Debug)]
pub struct GlobalCtx {
    ctx: SalsaAppContext<VisEvent, Error>,
}

impl SalsaContext<VisEvent, Error> for GlobalCtx {
    fn set_salsa_ctx(&mut self, app_ctx: SalsaAppContext<VisEvent, Error>) {
        self.ctx = app_ctx;
    }

    fn salsa_ctx(&self) -> &SalsaAppContext<VisEvent, Error> {
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

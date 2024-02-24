use std::time::Duration;

use crate::context::AutoClientContext;

pub struct TickRate(pub Duration);
impl FromContext for TickRate {
    fn from_context(context: &AutoClientContext) -> Self {
        Self(context.tick_rate)
    }
}

pub trait FromContext {
    fn from_context(context: &AutoClientContext) -> Self;
}

use std::time::Duration;

use crate::context::AutoClientContext;

pub struct TickRate(pub Duration);
impl FromContext for TickRate {
    fn from_context(context: &AutoClientContext) -> Self {
        Self(Duration::from_millis(context.tick_rate as u64))
    }
}

pub trait FromContext {
    fn from_context(context: &AutoClientContext) -> Self;
}

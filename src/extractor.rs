use std::time::Duration;

use crate::context::AutoClientContext;

pub struct TickRate(pub Duration);
impl<S> FromContext<S> for TickRate {
    fn from_context(context: &AutoClientContext, _: &S) -> Self {
        Self(context.tick_rate)
    }
}

pub trait FromContext<S> {
    fn from_context(context: &AutoClientContext, user_context: &S) -> Self;
}

pub struct State<S>(pub S);
impl<S> FromContext<S> for State<S>
where
    S: Clone,
{
    fn from_context(context: &AutoClientContext, user_context: &S) -> Self {
        Self(user_context.clone())
    }
}

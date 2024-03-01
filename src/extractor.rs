use std::time::Duration;

use crate::context::StateMachineContext;

pub struct TickRate(pub Duration);
impl<S> FromContext<S> for TickRate {
    fn from_context(context: &StateMachineContext, _: &S) -> Self {
        Self(context.tick_rate)
    }
}

pub trait FromContext<S> {
    fn from_context(context: &StateMachineContext, user_context: &S) -> Self;
}

pub struct State<S>(pub S);
impl<S> FromContext<S> for State<S>
where
    S: Clone,
{
    fn from_context(_context: &StateMachineContext, user_context: &S) -> Self {
        Self(user_context.clone())
    }
}

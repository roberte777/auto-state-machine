use std::time::Duration;

use crate::blocking::extractor::FromContext;

#[derive(Clone)]
pub enum LifeCycle {
    Running,
    Paused,
    Stopped,
}

// S is for user context (state)
// E is for States
#[derive(Clone)]
pub struct StateMachineContext {
    pub tick_rate: Duration,
    pub current_state: String,
    pub initial_state: String,
    pub life_cycle: LifeCycle,
}
impl<S> FromContext<S> for StateMachineContext {
    fn from_context(context: &StateMachineContext, _user_state: &S) -> Self {
        context.clone()
    }
}

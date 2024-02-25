use std::time::Duration;

use crate::extractor::FromContext;

#[derive(Clone)]
pub enum LifeCycle {
    Running,
    Paused,
    Stopped,
}

// S is for user context (state)
// E is for States
#[derive(Clone)]
pub struct AutoClientContext {
    pub tick_rate: Duration,
    pub current_state: String,
    pub initial_state: String,
    pub life_cycle: LifeCycle,
}
impl<S> FromContext<S> for AutoClientContext {
    fn from_context(context: &AutoClientContext, _user_state: &S) -> Self {
        context.clone()
    }
}

/// Wrapper type for erasure. T is the generic function arguments for F
pub struct Wrapper<T, F> {
    pub f: F,
    pub marker: std::marker::PhantomData<T>,
}

pub trait Callback<S>: Send + Sync {
    fn call(&self, context: &AutoClientContext, s: &mut S) -> String;
}
impl<F, S, T1> Callback<S> for Wrapper<(T1,), F>
where
    F: Fn(T1) -> String + Send + Sync,
    T1: FromContext<S> + Send + Sync,
{
    fn call(&self, context: &AutoClientContext, s: &mut S) -> String {
        (self.f)(T1::from_context(context, s))
    }
}
impl<F, T1, T2, S> Callback<S> for Wrapper<(T1, T2), F>
where
    F: Fn(T1, T2) -> String + Send + Sync,
    T1: FromContext<S> + Send + Sync,
    T2: FromContext<S> + Send + Sync,
{
    fn call(&self, context: &AutoClientContext, s: &mut S) -> String {
        (self.f)(T1::from_context(context, s), T2::from_context(context, s))
    }
}
pub type StoredCallback<S> = Box<dyn Callback<S>>;

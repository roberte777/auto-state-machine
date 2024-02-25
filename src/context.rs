use std::time::Duration;

use crate::extractor::FromContext;

// S is for user context (state)
// E is for States
#[derive(Clone)]
pub struct AutoClientContext {
    pub tick_rate: Duration,
    pub current_state: String,
    pub initial_state: String,
}
impl<S> FromContext<S> for AutoClientContext {
    fn from_context(context: &AutoClientContext, user_state: &S) -> Self {
        context.clone()
    }
}

/// Wrapper type for erasure. T is the generic function arguments for F
pub struct Wrapper<T, F> {
    pub f: F,
    pub marker: std::marker::PhantomData<T>,
}

pub trait Callback<S> {
    fn call(&self, context: &AutoClientContext, s: &S) -> String;
}
impl<F, S, T1> Callback<S> for Wrapper<(T1,), F>
where
    F: Fn(T1) -> String,
    T1: FromContext<S>,
{
    fn call(&self, context: &AutoClientContext, s: &S) -> String {
        (self.f)(T1::from_context(context, s))
    }
}
impl<F, T1, T2, S> Callback<S> for Wrapper<(T1, T2), F>
where
    F: Fn(T1, T2) -> String,
    T1: FromContext<S>,
    T2: FromContext<S>,
{
    fn call(&self, context: &AutoClientContext, s: &S) -> String {
        (self.f)(T1::from_context(context, s), T2::from_context(context, s))
    }
}
pub type StoredCallback<S> = Box<dyn Callback<S>>;

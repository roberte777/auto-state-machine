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
impl FromContext for AutoClientContext {
    fn from_context(context: &AutoClientContext) -> Self {
        context.clone()
    }
}

/// Wrapper type for erasure. T is the generic function arguments for F
pub struct Wrapper<T, F> {
    pub f: F,
    pub marker: std::marker::PhantomData<T>,
}

pub trait Callback<S> {
    fn call(&self, context: &AutoClientContext, s: S) -> String;
}
impl<F, S, T1> Callback<S> for Wrapper<(T1,), F>
where
    F: Fn(T1, S) -> String,
    T1: FromContext,
{
    fn call(&self, context: &AutoClientContext, s: S) -> String {
        (self.f)(T1::from_context(context), s)
    }
}
impl<F, T1, T2, S> Callback<S> for Wrapper<(T1, T2), F>
where
    F: Fn(T1, T2, S) -> String,
    T1: FromContext,
    T2: FromContext,
{
    fn call(&self, context: &AutoClientContext, s: S) -> String {
        (self.f)(T1::from_context(context), T2::from_context(context), s)
    }
}
pub type StoredCallback<S> = Box<dyn Callback<S>>;

// S is for user context (state)
// E is for States
#[derive(Clone)]
pub struct AutoClientContext {
    pub tick_rate: u32,
    pub current_state: String,
    pub initial_state: String,
}
impl FromContext for AutoClientContext {
    fn from_context(context: &AutoClientContext) -> Self {
        context.clone()
    }
}

pub trait FromContext {
    fn from_context(context: &AutoClientContext) -> Self;
}

pub trait Callbacks<T> {
    fn call(&self, context: &AutoClientContext) -> String;
}
impl<F, T> Callbacks<T> for F
where
    F: Fn(T) -> String,
    T: FromContext,
{
    fn call(&self, context: &AutoClientContext) -> String {
        self(T::from_context(context))
    }
}
impl<F, T1, T2> Callbacks<(T1, T2)> for F
where
    F: Fn(T1, T2) -> String,
    T1: FromContext,
    T2: FromContext,
{
    fn call(&self, context: &AutoClientContext) -> String {
        self(T1::from_context(context), T2::from_context(context))
    }
}

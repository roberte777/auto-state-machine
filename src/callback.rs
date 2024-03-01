use futures::future::BoxFuture;

use crate::context::StateMachineContext;
use crate::extractor::FromContext;
pub trait IntoCallback<Input, S> {
    type Callback: Callback<S>;

    fn into_callback(self) -> Self::Callback;
}
/// Wrapper type for erasure. T is the generic function arguments for F
pub struct Wrapper<T, F> {
    pub f: F,
    pub marker: std::marker::PhantomData<T>,
}

pub trait Callback<S>: Send + Sync {
    fn call(&self, context: &StateMachineContext, s: &mut S) -> BoxFuture<'static, String>;
}
pub type StoredCallback<S> = Box<dyn Callback<S>>;
macro_rules! impl_callback {
    (
        $($(
                $params:ident
        ),+)?
    ) => {
        impl<Fut, F, $($($params,)+)? S> Callback<S> for Wrapper<( $($($params,)+)? ), F>
        where
            F: Fn($($($params),+)?)-> Fut + Send + Sync,
            Fut: futures::Future<Output = String> + Send + 'static,
            $($($params: 'static + FromContext<S> + Send + Sync,)+)?
            S: 'static,
        {
            fn call(&self, context: &StateMachineContext, s: &mut S) -> BoxFuture<'static, String> {
                let fut = (self.f)($($($params::from_context(context, s)),+)?);
                Box::pin(async move {
                    let result = fut.await;
                    result
                })
            }
        }
    }
}
impl_callback!(T1);
impl_callback!(T1, T2);

macro_rules! impl_into_callback {
    (
        $($(
                $params:ident
        ),+)?
    ) => {
        impl<Fut, F, $($($params,)+)? S> IntoCallback<( $($($params,)+)? ), S> for F
        where
            F: Fn($($($params),+)?)-> Fut + Send + Sync,
            Fut: futures::Future<Output = String> + Send + 'static,
            $($($params: 'static + FromContext<S> + Send + Sync,)+)?
            S: 'static,
        {
            type Callback = Wrapper<( $($($params,)+)? ), Self>;

            fn into_callback(self) -> Self::Callback {
                Wrapper {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}
// impl_into_callback!();
impl_into_callback!(T1);
impl_into_callback!(T1, T2);
// impl_into_callback!(T1, T2, T3);
// impl_into_callback!(T1, T2, T3, T4);

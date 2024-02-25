use std::any::Any;

use crate::context::AutoClientContext;
use crate::extractor::FromContext;
pub trait IntoCallback<Input, S> {
    type Callback: Callback;

    fn into_callback(self) -> Self::Callback;
}
/// Wrapper type for erasure. T is the generic function arguments for F
pub struct Wrapper<T, F, S> {
    pub f: F,
    pub marker: std::marker::PhantomData<T>,
    pub _s: std::marker::PhantomData<S>,
}

pub trait Callback: Send + Sync {
    fn call(&self, context: &AutoClientContext, s: &mut Box<dyn Any>) -> String;
}
pub type StoredCallback = Box<dyn Callback>;
macro_rules! impl_callback {
    (
        $($(
                $params:ident
        ),+)?
    ) => {
        impl<F: Fn($($($params),+)?)->String + Send + Sync $(, $($params: 'static + FromContext<S> + Send + Sync),+ )?, S: Send + Sync + 'static> Callback for Wrapper<( $($($params,)+)? ), F, S> {

            fn call(&self, context: &AutoClientContext, s: &mut Box<dyn Any>) -> String {
                let s = s.downcast_mut::<S>().unwrap();
                (self.f)($($($params::from_context(context, s)),+)?)
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
        impl<F: Fn($($($params),+)?)->String + Send + Sync $(, $($params: 'static + FromContext<S> + Send + Sync),+ )?, S: Send + Sync +'static> IntoCallback<( $($($params,)+)? ), S> for F {
            type Callback = Wrapper<( $($($params,)+)? ), Self, S>;

            fn into_callback(self) -> Self::Callback {
                Wrapper {
                    f: self,
                    marker: Default::default(),
                    _s: Default::default(),
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

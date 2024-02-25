use crate::context::AutoClientContext;
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
    fn call(&self, context: &AutoClientContext, s: &mut S) -> String;
}
pub type StoredCallback<S> = Box<dyn Callback<S>>;
macro_rules! impl_callback {
    (
        $($(
                $params:ident
        ),+)?
    ) => {
        impl<F: Fn($($($params),+)?)->String + Send + Sync $(, $($params: 'static + FromContext<S> + Send + Sync),+ )?, S> Callback<S> for Wrapper<( $($($params,)+)? ), F> {

            fn call(&self, context: &AutoClientContext, s: &mut S) -> String {
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
        impl<F: Fn($($($params),+)?)->String + Send + Sync $(, $($params: 'static + FromContext<S> + Send + Sync),+ )?, S> IntoCallback<( $($($params,)+)? ), S> for F {
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

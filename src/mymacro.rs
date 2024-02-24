use crate::context::{Callback, Wrapper};
use crate::extractor::FromContext;
pub trait IntoCallback<Input> {
    type Callback: Callback;

    fn into_callback(self) -> Self::Callback;
}

macro_rules! impl_into_callback {
    (
        $($(
                $params:ident
        ),+)?
    ) => {
        impl<F: Fn($($($params),+)?)->String $(, $($params: 'static + FromContext),+ )?> IntoCallback<( $($($params,)+)? )> for F {
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

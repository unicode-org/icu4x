// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dynutil::UpcastDataPayload;
use crate::prelude::*;
use crate::yoke::*;
use alloc::boxed::Box;
use crabbake::{Bakeable, CrateEnv, TokenStream};

trait BakeableYoke {
    fn bake_yoke(&self, env: &CrateEnv) -> TokenStream;
}

impl<Y, C> BakeableYoke for Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a>,
    for<'a> <Y as Yokeable<'a>>::Output: Bakeable,
{
    fn bake_yoke(&self, ctx: &CrateEnv) -> TokenStream {
        self.get().bake(ctx)
    }
}

#[derive(yoke::Yokeable)]
pub struct CrabbakeBox {
    payload: Box<dyn BakeableYoke>,
    marker: Box<dyn Bakeable>,
}

impl<M> UpcastDataPayload<M> for CrabbakeMarker
where
    M: DataMarker + Default + Bakeable + 'static,
    for<'a> <M::Yokeable as Yokeable<'a>>::Output: Bakeable,
{
    fn upcast(other: DataPayload<M>) -> DataPayload<CrabbakeMarker> {
        DataPayload::from_owned(CrabbakeBox {
            payload: Box::new(other.yoke),
            // This is a bit naughty, we need the marker's type, but we're actually
            // baking its value. This works as long as all markers are unit structs.
            marker: Box::new(M::default()),
        })
    }
}

impl<M> DataPayload<M>
where
    M: DataMarker + Default + Bakeable + 'static,
    for<'a> <M::Yokeable as Yokeable<'a>>::Output: Bakeable,
{
    /// Converts a [`DataPayload`] into something that can be baked.
    ///
    /// See [`DataPayload::tokenize()`] for an example.
    pub fn into_bakeable(self) -> DataPayload<CrabbakeMarker> {
        CrabbakeMarker::upcast(self)
    }
}

impl DataPayload<CrabbakeMarker> {
    /// Serializes this [`DataPayload`]'s value and marker type into [`TokenStream`]s
    /// using their [`Bakeable`] implementations.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::hello_world::HelloWorldV1Marker;
    /// use icu_provider::prelude::*;
    /// # use crabbake::quote;
    /// # use std::collections::BTreeSet;
    ///
    /// // Create an example DataPayload
    /// let payload: DataPayload<HelloWorldV1Marker> = Default::default();
    ///
    /// let env = crabbake::CrateEnv::default();
    /// let (tokens, marker) = payload.into_bakeable().tokenize(&env);
    /// assert_eq!(
    ///     quote! {
    ///         ::icu_provider::hello_world::HelloWorldV1 {
    ///             message: ::alloc::borrow::Cow::Borrowed("(und) Hello World"),
    ///         }
    ///     }
    ///     .to_string(),
    ///     tokens.to_string()
    /// );
    /// assert_eq!(
    ///     quote! { ::icu_provider::hello_world::HelloWorldV1Marker }.to_string(),
    ///     marker.to_string()
    /// );
    /// assert_eq!(
    ///     env.into_iter().collect::<BTreeSet<_>>(),
    ///     ["icu_provider", "alloc"]
    ///         .into_iter()
    ///         .collect::<BTreeSet<_>>()
    /// );
    /// ```
    pub fn tokenize(&self, env: &CrateEnv) -> (TokenStream, TokenStream) {
        (
            self.get().payload.bake_yoke(env),
            self.get().marker.bake(env),
        )
    }
}

/// Marker type for [`CrabbakeBox`].
#[allow(clippy::exhaustive_structs)] // marker type
pub struct CrabbakeMarker {}

impl DataMarker for CrabbakeMarker {
    type Yokeable = CrabbakeBox;
}

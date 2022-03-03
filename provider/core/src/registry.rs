// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use core::fmt::Debug;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;

pub trait KeyRegistry {
    // Calls `ConcreteDataCallback::callback::<M>();` for the appropriate marker type M
    fn run_with_concrete_data<R>(
        &self,
        key: ResourceKey,
        callback: &mut impl ConcreteDataCallback<R>,
    ) -> Result<R, DataError>;
}

pub trait ConcreteDataCallback<R> {
    fn callback<M: DataMarker + 'static>(&mut self) -> Result<R, DataError>
    where
        for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: serde::Serialize,
        for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::Deserialize<'de>,
        for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Debug;
}

#[macro_export]
macro_rules! define_key_registry(
    ($registry:ident, $($key:path => $marker:path),*) => {
        pub struct $registry;

        impl $crate::registry::KeyRegistry for $registry {
            fn run_with_concrete_data<R>(&self, key: ResourceKey, callback: &mut impl ConcreteDataCallback<R>) -> Result<R, DataError> {
                $(
                    if key.get_hash() == $key.get_hash() {
                        return callback.callback::<$marker>()
                    }

                )*

                Err(DataErrorKind::MissingResourceKey.with_key(key))
            }
        }
    }
);

#[cfg(all(test, feature = "deserialize_postcard_07"))]
mod tests {
    use super::*;
    use crate::hello_world::*;
    use crate::serde::SerializeMarker;
    use core::fmt::Write;
    use icu_locid_macros::langid;

    define_key_registry!(HelloRegistry, HelloWorldV1Marker::KEY => HelloWorldV1Marker);

    #[test]
    fn test_registry_simple() {
        let provider = HelloWorldProvider::new_with_placeholder_data();
        erased_test_registry_simple(provider);
    }

    // Separate function so that we can assume the data marker type has been erased alread
    fn erased_test_registry_simple(provider: impl DynProvider<SerializeMarker>) {
        let request = DataRequest {
            options: langid!("bn").into(),
            metadata: Default::default(),
        };
        let payload = provider
            .load_payload(HelloWorldV1Marker::KEY, &request)
            .unwrap()
            .payload
            .unwrap();
        let mut callback = MyHelloCallback(payload);
        let value = HelloRegistry
            .run_with_concrete_data(HelloWorldV1Marker::KEY, &mut callback)
            .unwrap();

        assert_eq!(value, "HelloWorldV1 { message: \"ওহে বিশ\\u{9cd}ব\" }");
    }

    struct MyHelloCallback(DataPayload<SerializeMarker>);

    impl ConcreteDataCallback<String> for MyHelloCallback {
        fn callback<M: DataMarker>(&mut self) -> Result<String, DataError>
        where
            for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: serde::Serialize,
            for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::Deserialize<'de>,
            for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Debug,
        {
            let mut s = postcard::Serializer {
                output: postcard::flavors::AllocVec(vec![]),
            };

            {
                let mut erased_s = Box::new(<dyn erased_serde::Serializer>::erase(&mut s));
                self.0.serialize(&mut *erased_s)?;
            }

            let reified_data: YokeTraitHack<<M::Yokeable as Yokeable>::Output> =
                postcard::from_bytes(&s.output.0)?;

            let mut out = String::new();
            // We use Debug here since we're trying to test that concrete impls can be called in generic contexts
            // so we don't want to reference HelloWorld here
            write!(&mut out, "{:?}", reified_data).unwrap();
            Ok(out)
        }
    }
}

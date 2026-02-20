// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Macros for delegating impls to inner data providers.

/// Delegate an impl of `DataProvider<M>` to a field of a struct.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::*;
/// use icu_locale::locale;
///
/// struct Wrap(HelloWorldProvider);
///
/// // Delegate `DataProvider<HelloWorldV1>` to the field `self.0`
/// icu_provider_adapters::delegate::data_provider_to_field!(Wrap, HelloWorldV1, &self.0);
///
/// // Test that it works
/// let wrap = Wrap(HelloWorldProvider::default());
/// HelloWorldFormatter::try_new_unstable(&wrap, locale!("de").into()).unwrap();
/// ```
///
/// Also works if the field is a [`BufferProvider`]:
///
/// ```
/// use icu_provider::hello_world::*;
/// use icu_locale::locale;
///
/// struct Wrap(HelloWorldJsonProvider);
///
/// // Delegate `DataProvider<HelloWorldV1>` to the field `self.0`, calling `as_deserializing()` on the field
/// icu_provider_adapters::delegate::data_provider_to_field!(Wrap, HelloWorldV1, &self.0.as_deserializing());
///
/// // Test that it works
/// let wrap = Wrap(HelloWorldProvider::default().into_json_provider());
/// HelloWorldFormatter::try_new_unstable(&wrap, locale!("de").into()).unwrap();
/// ```
///
/// [`BufferProvider`]: icu_provider::prelude::BufferProvider
#[doc(hidden)] // macro
#[macro_export]
macro_rules! __data_provider_to_field {
    ($provider:path, $marker:path, &self.$field:tt.as_deserializing()) => {
        impl $crate::icu_provider::DataProvider<$marker> for $provider {
            fn load(&self, req: $crate::icu_provider::DataRequest) -> Result<$crate::icu_provider::DataResponse<$marker>, $crate::icu_provider::DataError> {
                let provider = $crate::icu_provider::prelude::AsDeserializingBufferProvider::as_deserializing(&self.$field);
                $crate::icu_provider::DataProvider::<$marker>::load(&provider, req)
            }
        }
    };
    ($provider:path, $marker:path, &self.$field:tt) => {
        impl $crate::icu_provider::DataProvider<$marker> for $provider {
            fn load(&self, req: $crate::icu_provider::DataRequest) -> Result<$crate::icu_provider::DataResponse<$marker>, $crate::icu_provider::DataError> {
                $crate::icu_provider::DataProvider::<$marker>::load(&self.$field, req)
            }
        }
    };
}

#[doc(inline)]
pub use __data_provider_to_field as data_provider_to_field;

#[cfg(test)]
mod tests {
    use icu_locale::locale;
    use icu_provider::hello_world::{
        HelloWorldFormatter, HelloWorldJsonProvider, HelloWorldProvider, HelloWorldV1,
    };

    use super::*;

    #[test]
    fn test_delegate() {
        struct Wrap(HelloWorldProvider);
        data_provider_to_field!(Wrap, HelloWorldV1, &self.0);

        let hello1 = HelloWorldProvider::default();
        let wrap = Wrap(hello1);

        let formatter = HelloWorldFormatter::try_new_unstable(&wrap, locale!("de").into()).unwrap();
        assert_eq!(formatter.format_to_string(), "Hallo Welt");
    }

    #[test]
    fn test_delegate_to_buffer() {
        struct Wrap(HelloWorldJsonProvider);
        data_provider_to_field!(Wrap, HelloWorldV1, &self.0.as_deserializing());

        let hello1 = HelloWorldProvider::default().into_json_provider();
        let wrap = Wrap(hello1);

        let formatter = HelloWorldFormatter::try_new_unstable(&wrap, locale!("de").into()).unwrap();
        assert_eq!(formatter.format_to_string(), "Hallo Welt");
    }
}
